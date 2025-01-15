//! Data structures and functions specific to `xvc file list`
//!
//! - [ListCLI] defines the command line options
//! - [cmd_list]  is the entry point to run the command

use crate::common::{load_targets_from_store, targets_from_disk, FileTextOrBinary};
use crate::error::Error;
use crate::Result;
use anyhow::anyhow;
use chrono;
use clap::Parser;
use clap_complete::ArgValueCompleter;
use xvc_core::util::completer::strum_variants_completer;

use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::fmt::Formatter;
use std::path::Path;
use std::str::FromStr;
use std::time::SystemTime;
use strum_macros::{Display as EnumDisplay, EnumString, VariantNames};
use xvc_config::{conf, FromConfigKey, UpdateFromXvcConfig};
use xvc_core::types::xvcdigest::DIGEST_LENGTH;
use xvc_core::{
    ContentDigest, HashAlgorithm, RecheckMethod, XvcFileType, XvcMetadata, XvcPath, XvcRoot,
};
use xvc_ecs::{HStore, XvcEntity, XvcStore};
use xvc_logging::{error, output, XvcOutputSender};

/// Format specifier for file list columns
#[derive(Debug, Clone, EnumString, EnumDisplay, PartialEq, Eq)]
pub enum ListColumn {
    /// Column for the actual content digest (base64 encoded).
    #[strum(serialize = "acd64")]
    ActualContentDigest64,

    /// Column for the actual content digest (base8 encoded).
    #[strum(serialize = "acd8")]
    ActualContentDigest8,

    /// Column for the actual file type.
    #[strum(serialize = "aft")]
    ActualFileType,

    /// Column for the actual size of the file.
    #[strum(serialize = "asz")]
    ActualSize,

    /// Column for the actual timestamp of the file.
    #[strum(serialize = "ats")]
    ActualTimestamp,

    /// Column for the name of the file.
    #[strum(serialize = "name")]
    Name,

    /// Column for the cache status of the file.
    #[strum(serialize = "cst")]
    CacheStatus,

    /// Column for the recorded recheck method.
    #[strum(serialize = "rrm")]
    RecordedRecheckMethod,

    /// Column for the recorded content digest (base64 encoded).
    #[strum(serialize = "rcd64")]
    RecordedContentDigest64,

    /// Column for the recorded content digest (base8 encoded).
    #[strum(serialize = "rcd8")]
    RecordedContentDigest8,

    /// Column for the recorded size of the file.
    #[strum(serialize = "rsz")]
    RecordedSize,

    /// Column for the recorded timestamp of the file.
    #[strum(serialize = "rts")]
    RecordedTimestamp,

    /// Column for a literal string value.
    #[strum(disabled)]
    Literal(String),
}

/// Represents the format of a list, including the columns to be displayed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListFormat {
    /// A vector of [ListColumn] enums representing the columns in the table.
    pub columns: Vec<ListColumn>,
}

impl FromStr for ListFormat {
    type Err = crate::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut columns = Vec::new();
        for begin_marker in s.split("{{") {
            let items = begin_marker.split("}}");
            for item in items {
                if let Ok(col) = item.parse::<ListColumn>() {
                    columns.push(col);
                } else {
                    columns.push(ListColumn::Literal(item.to_string()));
                }
            }
        }
        Ok(Self { columns })
    }
}

conf!(ListFormat, "file.list.format");

/// Specify how to sort file list
#[derive(Debug, Copy, Clone, EnumString, EnumDisplay, PartialEq, Eq, VariantNames)]
pub enum ListSortCriteria {
    #[strum(serialize = "none")]
    /// No sorting
    None,
    #[strum(serialize = "name-asc")]
    /// Sort by name in ascending order
    NameAsc,
    #[strum(serialize = "name-desc")]
    /// Sort by name in descending order
    NameDesc,
    #[strum(serialize = "size-asc")]
    /// Sort by size in ascending order
    SizeAsc,
    #[strum(serialize = "size-desc")]
    /// Sort by size in descending order
    SizeDesc,
    #[strum(serialize = "t-asc", serialize = "timestamp-asc", serialize = "ts-asc")]
    /// Sort by timestamp in ascending order
    TimestampAsc,
    #[strum(
        serialize = "t-desc",
        serialize = "timestamp-desc",
        serialize = "ts-desc"
    )]
    /// Sort by timestamp in descending order
    TimestampDesc,
}
conf!(ListSortCriteria, "file.list.sort");

/// A single item in the list output
#[derive(Debug, Clone, PartialEq)]
pub struct ListRow {
    /// The actual (on-disk) content digest of the file
    pub actual_content_digest_str: String,
    /// The actual (on-disk) file size
    pub actual_size: u64,
    /// The actual (on-disk) file size as a string
    pub actual_size_str: String,
    /// The actual (on-disk) file modification timestamp
    pub actual_timestamp: SystemTime,
    /// The actual (on-disk) file modification timestamp as a string
    pub actual_timestamp_str: String,
    /// The actual (on-disk) file type
    pub actual_file_type: String,

    /// The basename of the file
    pub name: String,
    /// The cache status of the file
    pub cache_status: String,

    /// The recheck method used to link to the cached file
    pub recorded_recheck_method: String,
    /// The recorded content digest of the file
    pub recorded_content_digest_str: String,
    /// The recorded size of the file
    pub recorded_size: u64,
    /// The recorded size of the file as a string
    pub recorded_size_str: String,
    /// The recorded timestamp of the file
    // FIXME: This can be used as a separate field to sort in the future
    #[allow(dead_code)]
    pub recorded_timestamp: SystemTime,
    /// The recorded timestamp of the file as a string
    pub recorded_timestamp_str: String,
}

impl ListRow {
    fn new(path_prefix: &Path, path_match: PathMatch) -> Result<Self> {
        let actual_file_type =
            String::from(if let Some(actual_metadata) = path_match.actual_metadata {
                match actual_metadata.file_type {
                    XvcFileType::Missing => "X",
                    XvcFileType::File => "F",
                    XvcFileType::Directory => "D",
                    XvcFileType::Symlink => "S",
                    XvcFileType::Hardlink => "H",
                    XvcFileType::Reflink => "R",
                }
            } else {
                "X"
            });

        let recorded_recheck_method =
            if let Some(recheck_method) = path_match.recorded_recheck_method {
                format_recheck_method(recheck_method)
            } else {
                "X".to_string()
            };

        let actual_content_digest_str = match path_match.actual_digest {
            Some(digest) => format!("{}", digest),
            None => str::repeat(" ", DIGEST_LENGTH * 2),
        };

        let actual_size = path_match
            .actual_metadata
            .and_then(|md| md.size)
            .unwrap_or(0);

        let actual_size_str = format_size(path_match.actual_metadata.and_then(|md| md.size));

        let actual_timestamp = path_match
            .actual_metadata
            .and_then(|md| md.modified)
            .unwrap_or(SystemTime::UNIX_EPOCH);

        let actual_timestamp_str =
            format_timestamp(path_match.actual_metadata.and_then(|md| md.modified));

        let recorded_size = path_match
            .recorded_metadata
            .and_then(|md| md.size)
            .unwrap_or(0);

        let recorded_size_str = format_size(path_match.recorded_metadata.and_then(|md| md.size));

        let recorded_timestamp = path_match
            .recorded_metadata
            .and_then(|md| md.modified)
            .unwrap_or(SystemTime::UNIX_EPOCH);

        let recorded_timestamp_str =
            format_timestamp(path_match.recorded_metadata.and_then(|md| md.modified));

        let recorded_content_digest_str = match path_match.recorded_digest {
            Some(digest) => format!("{}", digest),
            None => str::repeat(" ", DIGEST_LENGTH * 2),
        };

        let name = if let Some(ap) = path_match.actual_path {
            ap.strip_prefix(path_prefix.to_string_lossy().as_ref())
                .map_err(|e| Error::RelativeStripPrefixError { e })?
                .to_string()
        } else if let Some(rp) = path_match.recorded_path {
            rp.strip_prefix(path_prefix.to_string_lossy().as_ref())
                .map_err(|e| Error::RelativeStripPrefixError { e })?
                .to_string()
        } else {
            return Err(anyhow!("No actual or recorded path for {:?}", path_match).into());
        };

        // We don't consider subsecond differences to be significant.
        let cache_status = if path_match.actual_metadata.is_some() {
            if path_match.recorded_metadata.is_some() {
                // We use seconds resolution for file system changes not to change results
                let actual_secs = actual_timestamp
                    .duration_since(SystemTime::UNIX_EPOCH)?
                    .as_secs();
                let recorded_secs = recorded_timestamp
                    .duration_since(SystemTime::UNIX_EPOCH)?
                    .as_secs();
                match actual_secs.cmp(&recorded_secs) {
                    std::cmp::Ordering::Less => "<".to_string(),
                    std::cmp::Ordering::Greater => ">".to_string(),
                    std::cmp::Ordering::Equal => "=".to_string(),
                }
            } else {
                "X".to_string()
            }
        } else {
            "?".to_string()
        };

        Ok(ListRow {
            actual_content_digest_str,
            actual_file_type,
            actual_size,
            actual_size_str,
            actual_timestamp,
            actual_timestamp_str,
            name,
            cache_status,
            recorded_recheck_method,
            recorded_content_digest_str,
            recorded_size,
            recorded_size_str,
            recorded_timestamp,
            recorded_timestamp_str,
        })
    }
}

fn format_recheck_method(recheck_method: RecheckMethod) -> String {
    match recheck_method {
        RecheckMethod::Copy => "C".to_string(),
        RecheckMethod::Symlink => "S".to_string(),
        RecheckMethod::Hardlink => "H".to_string(),
        RecheckMethod::Reflink => "R".to_string(),
    }
}

/// Formats the timestamp with "%Y-%m-%d %H:%M:%S" if there is Some,
/// or returns a corresponding string of spaces.
pub fn format_timestamp(timestamp: Option<SystemTime>) -> String {
    match timestamp {
        Some(timestamp) => {
            let timestamp = chrono::DateTime::<chrono::Utc>::from(timestamp);
            format!("{}", timestamp.format("%Y-%m-%d %H:%M:%S"))
        }
        None => "                   ".to_string(),
    }
}

/// Format size in human readable form, and shows the small changes.
///
/// For files larger than 1MB, it shows the last 3 digits, so that small changes are visible.
///
/// MB, GB, TB are used for sizes larger than 1MB, 1GB, 1TB respectively
/// Calculations for these are done with 1024 as base, not 1000.
///
/// Returns a string of spaces with the same size of column if size is None.
pub fn format_size(size: Option<u64>) -> String {
    match size {
        Some(size) => {
            if size < 1024 * 1024 {
                format!("{:11}", size)
            } else if size < 1024 * 1024 * 1024 {
                format!("{:>4}MB.{}", size / 1024 / 1024, size % 1000)
            } else if size < 1024 * 1024 * 1024 * 1024 {
                format!("{:>4}GB.{}", size / 1024 / 1024 / 1024, size % 1000)
            } else {
                format!("{:>4}TB.{}", size / 1024 / 1024 / 1024 / 1024, size % 1000)
            }
        }
        None => "           ".to_owned(),
    }
}

#[derive(Debug, Clone)]
struct PathMatch {
    xvc_entity: Option<XvcEntity>,
    actual_path: Option<XvcPath>,
    actual_metadata: Option<XvcMetadata>,
    actual_digest: Option<ContentDigest>,
    recorded_path: Option<XvcPath>,
    recorded_metadata: Option<XvcMetadata>,
    recorded_digest: Option<ContentDigest>,
    recorded_recheck_method: Option<RecheckMethod>,
}

/// All rows of the file list and its format and sorting criteria
#[derive(Debug, Clone, PartialEq)]
pub struct ListRows {
    /// How to format the file row. See [ListColumn] for the available columns.
    pub format: ListFormat,
    /// How to sort the list. See [ListSortCriteria] for the available criteria.
    pub sort_criteria: ListSortCriteria,
    /// All elements of the file list
    pub rows: Vec<ListRow>,
}

impl ListRows {
    /// Create a new table with the specified params and sort it
    pub fn new(format: ListFormat, sort_criteria: ListSortCriteria, rows: Vec<ListRow>) -> Self {
        let mut s = Self {
            format,
            sort_criteria,
            rows,
        };
        sort_list_rows(&mut s);
        s
    }

    /// Create an empty table without any rows, format or sorting criteria
    pub fn empty() -> Self {
        Self {
            format: ListFormat { columns: vec![] },
            sort_criteria: ListSortCriteria::None,
            rows: vec![],
        }
    }

    /// Number if file lines in the table
    pub fn total_lines(&self) -> usize {
        self.rows.len()
    }

    /// Total size of the files in the table
    pub fn total_actual_size(&self) -> u64 {
        self.rows.iter().fold(0u64, |tot, r| tot + r.actual_size)
    }

    /// Total size of the recorded files in the table
    pub fn total_cached_size(&self) -> u64 {
        let mut cached_sizes = HashMap::<String, u64>::new();
        self.rows.iter().for_each(|r| {
            if !r.recorded_content_digest_str.trim().is_empty() {
                cached_sizes.insert(r.recorded_content_digest_str.to_string(), r.recorded_size);
            }
        });

        cached_sizes.values().sum()
    }
}

/// Print a single row from the given element and the format
pub fn build_row(row: &ListRow, format: &ListFormat) -> String {
    let mut output = String::new();
    for column in &format.columns {
        match column {
            ListColumn::RecordedRecheckMethod => output.push_str(&row.recorded_recheck_method),
            ListColumn::ActualFileType => output.push_str(&row.actual_file_type),
            ListColumn::ActualSize => output.push_str(&row.actual_size_str),
            ListColumn::ActualContentDigest64 => output.push_str(&row.actual_content_digest_str),
            ListColumn::ActualContentDigest8 => {
                output.push_str(if row.actual_content_digest_str.len() >= 8 {
                    &row.actual_content_digest_str[..8]
                } else {
                    &row.actual_content_digest_str
                })
            }
            ListColumn::ActualTimestamp => output.push_str(&row.actual_timestamp_str),
            ListColumn::Name => output.push_str(&row.name),
            ListColumn::RecordedSize => output.push_str(&row.recorded_size_str),
            ListColumn::RecordedContentDigest64 => {
                output.push_str(&row.recorded_content_digest_str)
            }
            ListColumn::RecordedContentDigest8 => {
                output.push_str(if row.recorded_content_digest_str.len() >= 8 {
                    &row.recorded_content_digest_str[..8]
                } else {
                    &row.recorded_content_digest_str
                })
            }
            ListColumn::RecordedTimestamp => output.push_str(&row.recorded_timestamp_str),
            ListColumn::CacheStatus => output.push_str(&row.cache_status),
            ListColumn::Literal(literal) => output.push_str(literal),
        }
    }
    output
}

/// Fn type to decouple the build_row function from the build_table function
type BuildRowFn = Box<dyn Fn(&ListRow, &ListFormat) -> String>;

/// Build a table from the list of rows
pub fn build_table(list_rows: &ListRows, build_row: BuildRowFn) -> String {
    let mut output = String::new();

    let format = &list_rows.format;
    for row in list_rows.rows.iter() {
        let row_str = build_row(row, format);
        output.push_str(&row_str);
        output.push('\n');
    }

    output
}

fn add_summary_line(list_rows: &ListRows) -> String {
    let total_lines = list_rows.total_lines();
    let total_actual_size = format_size(Some(list_rows.total_actual_size()));
    let total_cached_size = format_size(Some(list_rows.total_cached_size()));

    // TODO: Add a format string to this output similar to files
    format!("Total #: {total_lines} Workspace Size: {total_actual_size} Cached Size: {total_cached_size}\n")
}

fn sort_list_rows(list_rows: &mut ListRows) {
    let row_cmp = match list_rows.sort_criteria {
        ListSortCriteria::NameAsc => |a: &ListRow, b: &ListRow| a.name.cmp(&b.name),
        ListSortCriteria::NameDesc => |a: &ListRow, b: &ListRow| b.name.cmp(&a.name),
        ListSortCriteria::SizeAsc => |a: &ListRow, b: &ListRow| a.actual_size.cmp(&b.actual_size),
        ListSortCriteria::SizeDesc => |a: &ListRow, b: &ListRow| b.actual_size.cmp(&a.actual_size),
        ListSortCriteria::TimestampAsc => {
            |a: &ListRow, b: &ListRow| a.actual_timestamp.cmp(&b.actual_timestamp)
        }
        ListSortCriteria::TimestampDesc => {
            |a: &ListRow, b: &ListRow| b.actual_timestamp.cmp(&a.actual_timestamp)
        }
        ListSortCriteria::None => |_: &ListRow, _: &ListRow| std::cmp::Ordering::Equal,
    };

    list_rows.rows.sort_unstable_by(row_cmp);
}

impl Display for ListRows {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", build_table(self, Box::new(build_row)))?;
        Ok(())
    }
}

/// List files and their actual and cached metadata.
///
/// By default, it produces a list of files and directories in the current
/// directory.
/// The list can be formatted by using the --format option.
/// The default format can be set in the config file.
///
///
/// The command doesn't compute the actual content digest if it's not requested
/// in the format string.
///
/// By default the list is not sorted.
/// You can use the --sort option to sort the list by name, size or timestamp.
///

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(rename_all = "kebab-case")]
pub struct ListCLI {
    /// A string for each row of the output table
    ///
    /// The following are the keys for each row:
    ///
    /// - {{acd8}}:  actual content digest from the workspace file. First 8 digits.
    /// - {{acd64}}:  actual content digest. All 64 digits.
    /// - {{aft}}:  actual file type. Whether the entry is a file (F), directory (D),
    ///   symlink (S), hardlink (H) or reflink (R).
    /// - {{asz}}:  actual size. The size of the workspace file in bytes. It uses MB,
    ///   GB and TB to represent sizes larger than 1MB.
    /// - {{ats}}:  actual timestamp. The timestamp of the workspace file.
    /// - {{name}}: The name of the file or directory.
    /// - {{cst}}:  cache status. One of "=", ">", "<", or "X" to show
    ///   whether the file timestamp is the same as the cached timestamp, newer,
    ///   older, and not tracked.
    /// - {{rcd8}}:  recorded content digest stored in the cache. First 8 digits.
    /// - {{rcd64}}:  recorded content digest stored in the cache. All 64 digits.
    /// - {{rrm}}:  recorded recheck method. Whether the entry is linked to the workspace
    ///   as a copy (C), symlink (S), hardlink (H) or reflink (R).
    /// - {{rsz}}:  recorded size. The size of the cached content in bytes. It uses
    ///   MB, GB and TB to represent sizes larged than 1MB.
    /// - {{rts}}:  recorded timestamp. The timestamp of the cached content.
    ///
    /// The default format can be set with file.list.format in the config file.
    ///
    /// TODO: Think how to add a completion to ListFormat
    #[arg(long, short = 'f', verbatim_doc_comment)]
    pub format: Option<ListFormat>,
    /// Sort criteria.
    ///
    /// It can be one of none (default), name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc.
    ///
    /// The default option can be set with file.list.sort in the config file.
    ///
    /// TODO: Add sort_criteria_completion
    #[arg(long, short = 's', add = ArgValueCompleter::new(strum_variants_completer::<ListSortCriteria>))]
    pub sort: Option<ListSortCriteria>,

    /// Don't show total number and size of the listed files.
    ///
    /// The default option can be set with file.list.no_summary in the config file.
    #[arg(long)]
    pub no_summary: bool,

    /// Don't hide directories
    ///
    /// Directories are not listed by default. This flag lists them.
    #[arg(long, short = 'd', aliases=&["show-dirs"])]
    pub show_directories: bool,

    /// Don't hide dot files
    ///
    /// If not supplied, hides dot files like .gitignore and .xvcignore
    #[arg(long, short = 'D')]
    pub show_dot_files: bool,

    /// List files tracked by Git.
    ///
    /// By default, Xvc doesn't list files tracked by Git. Supply this option to list them.
    #[arg(long)]
    pub include_git_files: bool,

    /// Files/directories to list.
    ///
    /// If not supplied, lists all files under the current directory.
    ///
    /// TODO: Add a tracked_targets completer
    #[arg()]
    pub targets: Option<Vec<String>>,
}

impl UpdateFromXvcConfig for ListCLI {
    fn update_from_conf(
        self,
        conf: &xvc_config::XvcConfig,
    ) -> xvc_config::error::Result<Box<Self>> {
        let no_summary = self.no_summary || conf.get_bool("file.list.no_summary")?.option;
        let show_dot_files =
            self.show_dot_files || conf.get_bool("file.list.show_dot_files")?.option;

        let format = self.format.unwrap_or_else(|| ListFormat::from_conf(conf));
        let sort_criteria = self
            .sort
            .unwrap_or_else(|| ListSortCriteria::from_conf(conf));
        let include_git_files =
            self.include_git_files || conf.get_bool("file.list.include_git_files")?.option;

        Ok(Box::new(Self {
            no_summary,
            show_dot_files,
            include_git_files,
            format: Some(format),
            sort: Some(sort_criteria),
            ..self
        }))
    }
}

/// ## Output Format
///
/// The default format for the output is as follows:
///
/// XY  <Timestamp>     <Size>     <Name>   <Digest>
///
/// X shows the recheck method from [RecheckMethod]
/// - C: Copy
/// - H: Hardlink
/// - S: Symlink
/// - R: Reflink
/// - D: Directory
/// - M: Missing (Not Tracked)
///
/// Y is the current cache status
/// - =: Recorded and actual file have the same timestamp
/// - >: Cached file is newer, xvc recheck to update the file
/// - <: File is newer, xvc carry-in to update the cache
///
/// TODO: - I: File is ignored
pub fn cmd_list(output_snd: &XvcOutputSender, xvc_root: &XvcRoot, cli_opts: ListCLI) -> Result<()> {
    // FIXME: `opts` shouldn't be sent to the inner function, but we cannot make sure that it's
    // updated from the config files in callers. A refactoring is good here.
    let conf = xvc_root.config();
    let opts = cli_opts.update_from_conf(conf)?;
    let no_summary = opts.no_summary;
    let list_rows = cmd_list_inner(output_snd, xvc_root, &opts)?;

    // TODO: All output should be produced in a central location with implemented traits.
    // [ListRows] could receive no_summary when it's built and implement Display
    output!(
        output_snd,
        "{}",
        build_table(&list_rows, Box::new(build_row))
    );
    if !no_summary {
        output!(output_snd, "{}", add_summary_line(&list_rows));
    }

    Ok(())
}

/// The actual implementation moved here to get the listed elements separately to be used in
/// desktop and server
pub fn cmd_list_inner(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    opts: &ListCLI,
) -> Result<ListRows> {
    let conf = xvc_root.config();

    let current_dir = conf.current_dir()?;
    let filter_git_paths = !opts.include_git_files;

    let all_from_disk = targets_from_disk(
        output_snd,
        xvc_root,
        current_dir,
        &opts.targets,
        filter_git_paths,
    )?;

    let from_disk = filter_xvc_path_metadata_map(all_from_disk, opts);

    let from_store = load_targets_from_store(output_snd, xvc_root, current_dir, &opts.targets)?;

    let xvc_metadata_store = xvc_root.load_store::<XvcMetadata>()?;

    let recheck_method_store = xvc_root.load_store::<RecheckMethod>()?;

    let filter_keys = filter_xvc_path_xvc_metadata_stores(&from_store, &xvc_metadata_store, opts);

    let from_store = from_store.subset(filter_keys.iter().copied())?;
    let xvc_metadata_store = xvc_metadata_store.subset(filter_keys.iter().copied())?;
    let recheck_method_store = recheck_method_store.subset(filter_keys.into_iter())?;

    let matches = match_store_and_disk_paths(
        from_disk,
        from_store,
        xvc_metadata_store,
        recheck_method_store,
    );

    let matches = if opts.format.as_ref().unwrap().columns.iter().any(|c| {
        *c == ListColumn::RecordedContentDigest64 || *c == ListColumn::RecordedContentDigest8
    }) {
        fill_recorded_content_digests(xvc_root, matches)?
    } else {
        matches
    };

    let matches =
        if opts.format.as_ref().unwrap().columns.iter().any(|c| {
            *c == ListColumn::ActualContentDigest64 || *c == ListColumn::ActualContentDigest8
        }) {
            let algorithm = HashAlgorithm::from_conf(conf);
            fill_actual_content_digests(output_snd, xvc_root, algorithm, matches)?
        } else {
            matches
        };

    let path_prefix = current_dir.strip_prefix(xvc_root.absolute_path())?;

    let rows = build_rows_from_matches(output_snd, matches, path_prefix);
    let format = opts
        .format
        .clone()
        .expect("Option must be filled at this point");
    let sort_criteria = opts.sort.expect("Option must be filled at this point");

    let list_rows = ListRows::new(format, sort_criteria, rows);
    Ok(list_rows)
}

fn build_rows_from_matches(
    output_snd: &XvcOutputSender,
    matches: Vec<PathMatch>,
    path_prefix: &Path,
) -> Vec<ListRow> {
    matches
        .into_iter()
        .filter_map(|pm| match ListRow::new(path_prefix, pm) {
            Ok(lr) => Some(lr),
            Err(e) => {
                error!(output_snd, "{}", e);
                None
            }
        })
        .collect()
}

fn fill_actual_content_digests(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    algorithm: HashAlgorithm,
    matches: Vec<PathMatch>,
) -> Result<Vec<PathMatch>> {
    let text_or_binary_store = xvc_root.load_store::<FileTextOrBinary>()?;
    Ok(matches
        .into_iter()
        .filter_map(|pm| {
            if pm
                .actual_path
                .as_deref()
                .and(pm.actual_metadata.map(|md| md.is_file()))
                == Some(true)
            {
                let actual_path = pm.actual_path.as_ref().unwrap();
                let path = actual_path.to_absolute_path(xvc_root);
                let text_or_binary = if let Some(xvc_entity) = pm.xvc_entity {
                    text_or_binary_store
                        .get(&xvc_entity)
                        .copied()
                        .unwrap_or_default()
                } else {
                    FileTextOrBinary::default()
                };

                match ContentDigest::new(&path, algorithm, text_or_binary.as_inner()) {
                    Ok(digest) => Some(PathMatch {
                        actual_digest: Some(digest),
                        ..pm
                    }),
                    Err(e) => {
                        error!(output_snd, "{}", e);
                        None
                    }
                }
            } else {
                Some(pm)
            }
        })
        .collect())
}

fn fill_recorded_content_digests(
    xvc_root: &std::sync::Arc<xvc_core::types::xvcroot::XvcRootInner>,
    matches: Vec<PathMatch>,
) -> Result<Vec<PathMatch>> {
    let content_digest_store = xvc_root.load_store::<ContentDigest>()?;
    let matches: Vec<PathMatch> = matches
        .into_iter()
        .map(|pm| {
            if let Some(xvc_entity) = pm.xvc_entity {
                let digest = content_digest_store.get(&xvc_entity).cloned();
                PathMatch {
                    recorded_digest: digest,
                    ..pm
                }
            } else {
                pm
            }
        })
        .collect();
    Ok(matches)
}

/// There are four groups of paths:
/// 1. Paths that are in the store and on disk and have identical metadata
/// 2. Paths that are in the store and on disk but have different metadata
/// 3. Paths that are in the store but not on disk
/// 4. Paths that are on disk but not in the store
fn match_store_and_disk_paths(
    from_disk: HashMap<XvcPath, XvcMetadata>,
    from_store: HStore<XvcPath>,
    stored_xvc_metadata: HStore<XvcMetadata>,
    stored_recheck_method: HStore<RecheckMethod>,
) -> Vec<PathMatch> {
    // Now match actual and recorded paths

    let mut matches = Vec::<PathMatch>::new();

    let mut found_entities = HashSet::<XvcEntity>::new();

    for (disk_xvc_path, disk_xvc_md) in from_disk {
        // Group 1 and Group 2
        if let Some(xvc_entity) = from_store.entity_by_value(&disk_xvc_path) {
            let recorded_metadata = stored_xvc_metadata.get(&xvc_entity).cloned();
            let recorded_path = from_store.get(&xvc_entity).cloned();
            let recorded_recheck_method = stored_recheck_method.get(&xvc_entity).cloned();
            found_entities.insert(xvc_entity);
            let pm = PathMatch {
                xvc_entity: Some(xvc_entity),
                actual_path: Some(disk_xvc_path),
                actual_metadata: Some(disk_xvc_md),
                // digests will be filled later if needed
                actual_digest: None,

                recorded_metadata,
                recorded_path,
                recorded_recheck_method,
                recorded_digest: None,
            };
            matches.push(pm);
        } else {
            // Group 4
            let pm = PathMatch {
                xvc_entity: None,
                actual_path: Some(disk_xvc_path),
                actual_metadata: Some(disk_xvc_md),
                // digests will be filled later if needed
                actual_digest: None,

                recorded_recheck_method: None,
                recorded_metadata: None,
                recorded_path: None,
                recorded_digest: None,
            };
            matches.push(pm);
        }
    }

    // Group 3
    let not_found_entities = from_store
        .keys()
        .copied()
        .filter(|xvc_entity| !found_entities.contains(xvc_entity))
        .collect::<Vec<_>>();

    for xvc_entity in &not_found_entities {
        let recorded_metadata = stored_xvc_metadata.get(xvc_entity).cloned();
        let recorded_path = from_store.get(xvc_entity).cloned();
        let recorded_recheck_method = stored_recheck_method.get(xvc_entity).cloned();
        let pm = PathMatch {
            xvc_entity: Some(*xvc_entity),
            actual_path: None,
            actual_metadata: None,
            // digests will be filled later if needed
            actual_digest: None,
            recorded_path,
            recorded_metadata,
            recorded_recheck_method,
            recorded_digest: None,
        };
        matches.push(pm);
    }
    matches
}

fn filter_xvc_path_metadata_map(
    all_from_disk: HashMap<XvcPath, XvcMetadata>,
    opts: &ListCLI,
) -> HashMap<XvcPath, XvcMetadata> {
    let filter_fn = match (opts.show_dot_files, opts.show_directories) {
        (true, true) => |_, _| true,

        (true, false) => |_, md: &XvcMetadata| !md.is_dir(),
        (false, true) => |path: &XvcPath, _| !(path.starts_with_str(".") || path.contains("./")),
        (false, false) => |path: &XvcPath, md: &XvcMetadata| {
            !(path.starts_with_str(".") || path.contains("./") || md.is_dir())
        },
    };

    all_from_disk
        .iter()
        .filter_map(|(path, md)| {
            if filter_fn(path, md) {
                Some((path.clone(), *md))
            } else {
                None
            }
        })
        .collect()
}

fn filter_xvc_path_xvc_metadata_stores(
    xvc_path_store: &HStore<XvcPath>,
    xvc_metadata_store: &XvcStore<XvcMetadata>,
    opts: &ListCLI,
) -> HashSet<XvcEntity> {
    let filter_fn = match (opts.show_dot_files, opts.show_directories) {
        (true, true) => |_, _| true,

        (true, false) => |_, md: &XvcMetadata| !md.is_dir(),
        (false, true) => |path: &XvcPath, _| !(path.starts_with_str(".") || path.contains("./")),
        (false, false) => |path: &XvcPath, md: &XvcMetadata| {
            !(path.starts_with_str(".") || path.contains("./") || md.is_dir())
        },
    };

    xvc_path_store
        .iter()
        .filter_map(|(xvc_entity, xvc_path)| {
            if let Some(xvc_metadata) = xvc_metadata_store.get(xvc_entity) {
                if filter_fn(xvc_path, xvc_metadata) {
                    Some(*xvc_entity)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}
