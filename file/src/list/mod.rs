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

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::str::FromStr;
use std::time::SystemTime;
use strum_macros::{Display as EnumDisplay, EnumString};
use xvc_config::{conf, FromConfigKey, UpdateFromXvcConfig};
use xvc_core::types::xvcdigest::DIGEST_LENGTH;
use xvc_core::{
    ContentDigest, HashAlgorithm, RecheckMethod, XvcFileType, XvcMetadata, XvcPath, XvcRoot,
};
use xvc_ecs::XvcEntity;
use xvc_logging::{error, output, watch, XvcOutputSender};

#[derive(Debug, Clone, EnumString, EnumDisplay, PartialEq, Eq)]
enum ListColumn {
    #[strum(serialize = "acd64")]
    ActualContentDigest64,
    #[strum(serialize = "acd8")]
    ActualContentDigest8,
    #[strum(serialize = "aft")]
    ActualFileType,
    #[strum(serialize = "asz")]
    ActualSize,
    #[strum(serialize = "ats")]
    ActualTimestamp,
    #[strum(serialize = "name")]
    Name,
    #[strum(serialize = "cst")]
    CacheStatus,
    #[strum(serialize = "rrm")]
    RecordedRecheckMethod,
    #[strum(serialize = "rcd64")]
    RecordedContentDigest64,
    #[strum(serialize = "rcd8")]
    RecordedContentDigest8,
    #[strum(serialize = "rsz")]
    RecordedSize,
    #[strum(serialize = "rts")]
    RecordedTimestamp,
    #[strum(disabled)]
    Literal(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ListFormat {
    columns: Vec<ListColumn>,
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

#[derive(Debug, Copy, Clone, EnumString, EnumDisplay, PartialEq, Eq)]
enum ListSortCriteria {
    #[strum(serialize = "none")]
    None,
    #[strum(serialize = "name-asc")]
    NameAsc,
    #[strum(serialize = "name-desc")]
    NameDesc,
    #[strum(serialize = "size-asc")]
    SizeAsc,
    #[strum(serialize = "size-desc")]
    SizeDesc,
    #[strum(serialize = "t-asc", serialize = "timestamp-asc", serialize = "ts-asc")]
    TimestampAsc,
    #[strum(
        serialize = "t-desc",
        serialize = "timestamp-desc",
        serialize = "ts-desc"
    )]
    TimestampDesc,
}
conf!(ListSortCriteria, "file.list.sort");

#[derive(Debug, Clone)]
struct ListRow {
    actual_content_digest_str: String,
    actual_size: u64,
    actual_size_str: String,
    actual_timestamp: SystemTime,
    actual_timestamp_str: String,
    actual_file_type: String,

    name: String,
    cache_status: String,

    recorded_recheck_method: String,
    recorded_content_digest_str: String,
    recorded_size: u64,
    recorded_size_str: String,
    // This can be used as a separate field to sort in the future
    #[allow(dead_code)]
    recorded_timestamp: SystemTime,
    recorded_timestamp_str: String,
}

impl ListRow {
    ///
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

        watch!(&path_prefix);
        let name = if let Some(ap) = path_match.actual_path {
            watch!(ap);
            ap.strip_prefix(&path_prefix.to_string_lossy().to_string())
                .map_err(|e| Error::RelativeStripPrefixError { e })?
                .to_string()
        } else if let Some(rp) = path_match.recorded_path {
            watch!(rp);
            rp.strip_prefix(&path_prefix.to_string_lossy().to_string())
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

#[derive(Debug, Clone)]
struct ListRows {
    format: ListFormat,
    sort_criteria: ListSortCriteria,
    rows: RefCell<Vec<ListRow>>,
}

impl ListRows {
    pub fn new(format: ListFormat, sort_criteria: ListSortCriteria, rows: Vec<ListRow>) -> Self {
        Self {
            format,
            sort_criteria,
            rows: RefCell::new(rows),
        }
    }

    fn build_row(&self, row: &ListRow) -> String {
        let mut output = String::new();
        for column in &self.format.columns {
            match column {
                ListColumn::RecordedRecheckMethod => output.push_str(&row.recorded_recheck_method),
                ListColumn::ActualFileType => output.push_str(&row.actual_file_type),
                ListColumn::ActualSize => output.push_str(&row.actual_size_str),
                ListColumn::ActualContentDigest64 => {
                    output.push_str(&row.actual_content_digest_str)
                }
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

    pub fn build_table(&self, print_summary: bool) -> String {
        let mut output = String::new();
        let row_cmp = |a: &ListRow, b: &ListRow| match self.sort_criteria {
            ListSortCriteria::NameAsc => a.name.cmp(&b.name),
            ListSortCriteria::NameDesc => b.name.cmp(&a.name),
            ListSortCriteria::SizeAsc => a.actual_size.cmp(&b.actual_size),
            ListSortCriteria::SizeDesc => b.actual_size.cmp(&a.actual_size),
            ListSortCriteria::TimestampAsc => a.actual_timestamp.cmp(&b.actual_timestamp),
            ListSortCriteria::TimestampDesc => b.actual_timestamp.cmp(&a.actual_timestamp),
            ListSortCriteria::None => std::cmp::Ordering::Equal,
        };
        if self.sort_criteria != ListSortCriteria::None {
            self.rows.borrow_mut().sort_unstable_by(row_cmp)
        }

        for row in self.rows.borrow().iter() {
            let row_str = self.build_row(row);
            output.push_str(&row_str);
            output.push('\n');
        }

        if print_summary {
            let total_lines = self.rows.borrow().len();
            let total_actual_size = format_size(Some(
                self.rows
                    .borrow()
                    .iter()
                    .fold(0u64, |tot, r| tot + r.actual_size),
            ));
            let mut cached_sizes = HashMap::<String, u64>::new();
            self.rows.borrow().iter().for_each(|r| {
                if !r.recorded_content_digest_str.trim().is_empty() {
                    cached_sizes.insert(r.recorded_content_digest_str.to_string(), r.recorded_size);
                }
            });

            let total_cached_size = format_size(Some(cached_sizes.values().sum()));
            output.push_str(
                &format!("Total #: {total_lines} Workspace Size: {total_actual_size} Cached Size: {total_cached_size}\n"),
            )
        }
        output
    }
}

impl Display for ListRows {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.build_table(true))?;
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
    /// - {{cst}}:  cache status. One of "=", ">", "<", "X", or "?" to show
    ///   whether the file timestamp is the same as the cached timestamp, newer,
    ///   older, not cached or not tracked.
    /// - {{rcd8}}:  recorded content digest stored in the cache. First 8 digits.
    /// - {{rcd64}}:  recorded content digest stored in the cache. All 64 digits.
    /// - {{rrm}}:  recorded recheck method. Whether the entry is linked to the workspace
    ///   as a copy (C), symlink (S), hardlink (H) or reflink (R).
    /// - {{rsz}}:  recorded size. The size of the cached content in bytes. It uses
    ///   MB, GB and TB to represent sizes larged than 1MB.
    /// - {{rts}}:  recorded timestamp. The timestamp of the cached content.
    ///
    /// The default format can be set with file.list.format in the config file.
    #[arg(long, short = 'f', verbatim_doc_comment)]
    format: Option<ListFormat>,
    /// Sort criteria.
    ///
    /// It can be one of none (default), name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc.
    ///
    /// The default option can be set with file.list.sort in the config file.
    #[arg(long, short = 's')]
    sort: Option<ListSortCriteria>,

    /// Don't show total number and size of the listed files.
    ///
    /// The default option can be set with file.list.no_summary in the config file.
    #[arg(long)]
    no_summary: bool,
    /// Files/directories to list.
    ///
    /// If not supplied, lists all files under the current directory.
    #[arg()]
    targets: Option<Vec<String>>,
}

impl UpdateFromXvcConfig for ListCLI {
    fn update_from_conf(
        self,
        conf: &xvc_config::XvcConfig,
    ) -> xvc_config::error::Result<Box<Self>> {
        let format = self.format.unwrap_or_else(|| ListFormat::from_conf(conf));
        let sort_criteria = self
            .sort
            .unwrap_or_else(|| ListSortCriteria::from_conf(conf));
        Ok(Box::new(Self {
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
/// TODO: - I: File is ignored

pub fn cmd_list(output_snd: &XvcOutputSender, xvc_root: &XvcRoot, cli_opts: ListCLI) -> Result<()> {
    let conf = xvc_root.config();
    let opts = cli_opts.update_from_conf(conf)?;

    let current_dir = conf.current_dir()?;

    let from_disk = targets_from_disk(xvc_root, current_dir, &opts.targets)?;
    watch!(from_disk);
    let from_store = load_targets_from_store(xvc_root, current_dir, &opts.targets)?;
    watch!(from_store);
    let stored_xvc_metadata = xvc_root.load_store::<XvcMetadata>()?;
    let stored_recheck_method = xvc_root.load_store::<RecheckMethod>()?;

    // Now match actual and recorded paths

    let mut matches = Vec::<PathMatch>::new();

    // There are four groups of paths:
    // 1. Paths that are in the store and on disk and have identical metadata
    // 2. Paths that are in the store and on disk but have different metadata
    // 3. Paths that are in the store but not on disk
    // 4. Paths that are on disk but not in the store

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

    watch!(matches);

    // Now fill in the digests if needed.
    // We use rec content digest to identify cache paths and calculate cache
    // size. So we always load and fill these values.
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

    // Do not calculate actual content hashes if it's not requested in the
    // format string.
    let matches =
        if opts.format.as_ref().unwrap().columns.iter().any(|c| {
            *c == ListColumn::ActualContentDigest64 || *c == ListColumn::ActualContentDigest8
        }) {
            let algorithm = HashAlgorithm::from_conf(conf);
            let text_or_binary_store = xvc_root.load_store::<FileTextOrBinary>()?;
            matches
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
                .collect()
        } else {
            matches
        };

    let path_prefix = current_dir.strip_prefix(xvc_root.absolute_path())?;

    let rows = matches
        .into_iter()
        .filter_map(|pm| match ListRow::new(path_prefix, pm) {
            Ok(lr) => Some(lr),
            Err(e) => {
                error!(output_snd, "{}", e);
                None
            }
        })
        .collect();

    let list_rows = ListRows::new(opts.format.unwrap(), opts.sort.unwrap(), rows);
    output!(output_snd, "{}", list_rows.build_table(!opts.no_summary));
    Ok(())
}
