//! Data structures and functions specific to `xvc file list`
//!
//! - [ListCLI] defines the command line options
//! - [cmd_list]  is the entry point to run the command
use crate::common::compare::{diff_cache_type, PathComparisonParams};
use crate::common::{calc_digest, targets_from_disk, targets_from_store, FileTextOrBinary};
use crate::error::Error;
use crate::{Result, XvcFileCLI};
use anyhow::anyhow;
use chrono;
use clap::Parser;
use crossbeam_channel::Sender;
use log::warn;
use serde::__private::de::Content;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::SystemTime;
use strum_macros::{Display as EnumDisplay, EnumString};
use xvc_config::{conf, FromConfigKey, UpdateFromXvcConfig, XvcConfig};
use xvc_core::{
    CacheType, ContentDigest, HashAlgorithm, TextOrBinary, XvcFileType, XvcMetadata, XvcPath,
    XvcRoot,
};
use xvc_ecs::XvcEntity;
use xvc_logging::{error, output, watch, XvcOutputLine};

#[derive(Debug, Clone, EnumString, EnumDisplay, PartialEq, Eq)]
enum ListColumn {
    #[strum(serialize = "acd")]
    ActualContentDigest,
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
    #[strum(serialize = "rct")]
    RecordedCacheType,
    #[strum(serialize = "rcd")]
    RecordedContentDigest,
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
            let mut items = begin_marker.split("}}");
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

    recorded_cache_type: String,
    recorded_content_digest_str: String,
    recorded_size: u64,
    recorded_size_str: String,
    recorded_timestamp: SystemTime,
    recorded_timestamp_str: String,
}

impl ListRow {
    ///
    fn new(xvc_root: &XvcRoot, path_prefix: &Path, path_match: PathMatch) -> Result<Self> {
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

        let recorded_cache_type = if let Some(cache_type) = path_match.recorded_cache_type {
            format_cache_type(cache_type)
        } else {
            "X".to_string()
        };

        let actual_content_digest_str = match path_match.actual_digest {
            Some(digest) => format!("{}", digest),
            None => "".to_string(),
        };

        let (actual_size, actual_size_str) =
            if let Some(actual_metadata) = path_match.actual_metadata {
                if let Some(size) = actual_metadata.size {
                    (size, format_size(size))
                } else {
                    (0, "".to_string())
                }
            } else {
                (0, "".to_string())
            };

        let (actual_timestamp, actual_timestamp_str) = if let Some(xmd) = path_match.actual_metadata
        {
            if let Some(timestamp) = xmd.modified {
                (timestamp, format_timestamp(timestamp))
            } else {
                (SystemTime::UNIX_EPOCH, "".to_string())
            }
        } else {
            (SystemTime::UNIX_EPOCH, "".to_string())
        };

        let (recorded_size, recorded_size_str) =
            if let Some(recorded_metadata) = path_match.recorded_metadata {
                if let Some(size) = recorded_metadata.size {
                    (size, format_size(size))
                } else {
                    (0, "".to_string())
                }
            } else {
                (0, "".to_string())
            };

        let (recorded_timestamp, recorded_timestamp_str) =
            if let Some(xmd) = path_match.recorded_metadata {
                if let Some(timestamp) = xmd.modified {
                    (timestamp, format_timestamp(timestamp))
                } else {
                    (SystemTime::UNIX_EPOCH, "".to_string())
                }
            } else {
                (SystemTime::UNIX_EPOCH, "".to_string())
            };

        let recorded_content_digest_str = match path_match.recorded_digest {
            Some(digest) => format!("{}", digest),
            None => "".to_string(),
        };

        watch!(&path_prefix);
        let name = if let Some(ap) = path_match.actual_path {
            watch!(ap);
            ap.strip_prefix(&path_prefix.to_string_lossy().to_string())
                .map_err(|e| Error::RelativeStripPrefixError { e })?
                .to_string()
        } else {
            if let Some(rp) = path_match.recorded_path {
                watch!(rp);
                rp.strip_prefix(&path_prefix.to_string_lossy().to_string())
                    .map_err(|e| Error::RelativeStripPrefixError { e })?
                    .to_string()
            } else {
                return Err(anyhow!("No actual or recorded path for {:?}", path_match).into());
            }
        };

        // We don't consider subsecond differences to be significant.
        let cache_status = if path_match.actual_metadata.is_some() {
            if path_match.recorded_metadata.is_some() {
                if actual_timestamp < recorded_timestamp {
                    if recorded_timestamp
                        .duration_since(actual_timestamp)?
                        .as_secs()
                        > 0
                    {
                        "<".to_string()
                    } else {
                        "=".to_string()
                    }
                } else if recorded_timestamp > actual_timestamp {
                    if actual_timestamp
                        .duration_since(recorded_timestamp)?
                        .as_secs()
                        > 0
                    {
                        ">".to_string()
                    } else {
                        "=".to_string()
                    }
                } else {
                    "=".to_string()
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
            recorded_cache_type,
            recorded_content_digest_str,
            recorded_size,
            recorded_size_str,
            recorded_timestamp,
            recorded_timestamp_str,
        })
    }
}

fn format_cache_type(cache_type: CacheType) -> String {
    match cache_type {
        CacheType::Copy => "C".to_string(),
        CacheType::Symlink => "S".to_string(),
        CacheType::Hardlink => "H".to_string(),
        CacheType::Reflink => "R".to_string(),
    }
}

fn format_timestamp(timestamp: SystemTime) -> String {
    let timestamp = chrono::DateTime::<chrono::Utc>::from(timestamp);
    format!("{}", timestamp.format("%Y-%m-%d %H:%M:%S"))
}

/// Format size in human readable form, and shows the small changes.
///
/// For files larger than 1MB, it shows the last 3 digits, so that small changes are visible.
///
/// MB, GB, TB are used for sizes larger than 1MB, 1GB, 1TB respectively
/// Calculations for these are done with 1024 as base, not 1000.
pub fn format_size(size: u64) -> String {
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

#[derive(Debug, Clone)]
struct PathMatch {
    xvc_entity: Option<XvcEntity>,
    actual_path: Option<XvcPath>,
    actual_metadata: Option<XvcMetadata>,
    actual_digest: Option<ContentDigest>,
    recorded_path: Option<XvcPath>,
    recorded_metadata: Option<XvcMetadata>,
    recorded_digest: Option<ContentDigest>,
    recorded_cache_type: Option<CacheType>,
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
                ListColumn::RecordedCacheType => output.push_str(&row.recorded_cache_type),
                ListColumn::ActualFileType => output.push_str(&row.actual_file_type),
                ListColumn::ActualSize => output.push_str(&format!("{:>20}", row.actual_size_str)),
                ListColumn::ActualContentDigest => {
                    output.push_str(&format!("{:>8}", row.actual_content_digest_str))
                }
                ListColumn::ActualTimestamp => {
                    output.push_str(&format!("{:>20}", row.actual_timestamp_str))
                }
                ListColumn::Name => output.push_str(&row.name),
                ListColumn::RecordedSize => {
                    output.push_str(&format!("{:>20}", row.recorded_size_str))
                }
                ListColumn::RecordedContentDigest => {
                    output.push_str(&format!("{:>8}", row.recorded_content_digest_str))
                }
                ListColumn::RecordedTimestamp => {
                    output.push_str(&format!("{:>20}", row.recorded_timestamp_str))
                }
                ListColumn::CacheStatus => output.push_str(&row.cache_status),
                ListColumn::Literal(literal) => output.push_str(&literal),
            }
        }
        output
    }

    pub fn build_table(&self, print_summary: bool) -> String {
        let mut output = String::new();
        let row_cmp = |a: &ListRow, b: &ListRow| match self.sort_criteria {
            ListSortCriteria::NameAsc => a.name.cmp(&b.name),
            ListSortCriteria::NameDesc => b.name.cmp(&a.name),
            ListSortCriteria::SizeAsc => a.recorded_size.cmp(&b.actual_size),
            ListSortCriteria::SizeDesc => b.recorded_size.cmp(&a.actual_size),
            ListSortCriteria::TimestampAsc => a.recorded_timestamp.cmp(&b.actual_timestamp),
            ListSortCriteria::TimestampDesc => b.recorded_timestamp.cmp(&a.actual_timestamp),
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
            let total_actual_size =
                format_size(self.rows.borrow().iter().fold(0u64, |tot, r| r.actual_size));
            let total_cached_size =
                format_size(self.rows.borrow().iter().fold(0u64, |tot, r| r.actual_size));
            output.push_str(
                &format!("Total #: {total_lines} Workspace: {total_actual_size} Cached: {total_cached_size}\n"),
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
    /// - {{acd}}:  actual content digest. The hash of the workspace file's content.
    /// - {{aft}}:  actual file type. Whether the entry is a file (F), directory (D),
    ///   symlink (S), hardlink (H) or reflink (R).
    /// - {{asz}}:  actual size. The size of the workspace file in bytes. It uses MB,
    ///   GB and TB to represent sizes larger than 1MB.
    /// - {{ats}}:  actual timestamp. The timestamp of the workspace file.
    /// - {{name}}: The name of the file or directory.
    /// - {{cst}}:  cache status. One of "=", ">", "<", "X", or "?" to show
    ///   whether the file timestamp is the same as the cached timestamp, newer,
    ///   older, not cached or not tracked.
    /// - {{rcd}}:  recorded content digest. The hash of the cached content.
    /// - {{rct}}:  recorded cache type. Whether the entry is linked to the workspace
    ///   as a copy (C), symlink (S), hardlink (H) or reflink (R).
    /// - {{rsz}}:  recorded size. The size of the cached content in bytes. It uses
    ///   MB, GB and TB to represent sizes larged than 1MB.
    /// - {{rts}}:  recorded timestamp. The timestamp of the cached content.
    ///
    /// The default format can be set with file.list.format in the config file.
    #[arg(long, short = 'f')]
    format: Option<ListFormat>,
    /// Sort column.
    ///
    /// It can be one of none, name-asc, name-desc, size-asc, size-desc, ts-asc, ts-desc.
    ///
    /// The default is name-desc
    /// The default option can be set with file.list.sort in the config file.
    #[arg(long, short = 's', alias = "sort")]
    sort_criteria: Option<ListSortCriteria>,
    /// Don't show total number and size of the listed files.
    /// The default option can be set with file.list.no_summary in the config file.
    #[arg(long)]
    no_summary: bool,
    /// Show directory contents recursively.
    /// The default option can be set with file.list.recursive in the config file.
    #[arg(long, short = 'R')]
    recursive: bool,
    /// Files/directories to list
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
            .sort_criteria
            .unwrap_or_else(|| ListSortCriteria::from_conf(conf));
        let current_dir = conf.current_dir()?;
        Ok(Box::new(Self {
            format: Some(format),
            sort_criteria: Some(sort_criteria),
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
/// X shows the cache type from [CacheType]
/// - C: Copy
/// - H: Hardlink
/// - S: Symlink
/// - R: Reflink
/// - D: Directory
/// - M: Missing (Not Tracked)
///
/// Y is the current cache status
/// - =: Recorded and actual file have the same timestamp
/// - >: Cached file is newer, xvc checkout to update the file
/// - <: File is newer, xvc commit to update the cache
/// TODO: - I: File is ignored

pub fn cmd_list(
    output_snd: &Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
    cli_opts: ListCLI,
) -> Result<()> {
    let conf = xvc_root.config();
    let opts = cli_opts.update_from_conf(conf)?;

    let current_dir = conf.current_dir()?;

    let from_disk = targets_from_disk(xvc_root, current_dir, &opts.targets)?;
    let from_store = targets_from_store(xvc_root, current_dir, &opts.targets)?;
    let stored_xvc_metadata = xvc_root.load_store::<XvcMetadata>()?;
    let stored_cache_type = xvc_root.load_store::<CacheType>()?;

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
            let recorded_md = stored_xvc_metadata.get(&xvc_entity).unwrap();
            let recorded_path = from_store.get(&xvc_entity).unwrap();
            let recorded_cache_type = stored_cache_type.get(&xvc_entity).unwrap();
            found_entities.insert(xvc_entity.clone());
            let pm = PathMatch {
                xvc_entity: Some(xvc_entity.clone()),
                actual_path: Some(disk_xvc_path),
                actual_metadata: Some(disk_xvc_md),
                // digests will be filled later if needed
                actual_digest: None,

                recorded_metadata: Some(recorded_md.clone()),
                recorded_path: Some(recorded_path.clone()),
                recorded_cache_type: Some(recorded_cache_type.clone()),
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

                recorded_cache_type: None,
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
        let recorded_md = stored_xvc_metadata.get(&xvc_entity).unwrap();
        let recorded_path = from_store.get(&xvc_entity).unwrap();
        let recorded_cache_type = stored_cache_type.get(&xvc_entity).unwrap();
        let pm = PathMatch {
            xvc_entity: Some(xvc_entity.clone()),
            actual_path: None,
            actual_metadata: None,
            // digests will be filled later if needed
            actual_digest: None,
            recorded_path: Some(recorded_path.clone()),
            recorded_metadata: Some(recorded_md.clone()),
            recorded_cache_type: Some(recorded_cache_type.clone()),
            recorded_digest: None,
        };
        matches.push(pm);
    }

    // Now fill in the digests if needed
    // unwrap shouldn't panic as we fill the options in from_conf.
    let matches = if opts
        .format
        .as_ref()
        .unwrap()
        .columns
        .contains(&ListColumn::RecordedContentDigest)
    {
        let content_digest_store = xvc_root.load_store::<ContentDigest>()?;
        matches
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
            .collect()
    } else {
        matches
    };

    let matches = if opts
        .format
        .as_ref()
        .unwrap()
        .columns
        .contains(&ListColumn::ActualContentDigest)
    {
        let algorithm = HashAlgorithm::from_conf(conf);
        let text_or_binary_store = xvc_root.load_store::<FileTextOrBinary>()?;
        matches
            .into_iter()
            .filter_map(|pm| {
                if pm.actual_path.is_some()
                    && pm.actual_metadata.is_some()
                    && pm.actual_metadata.unwrap().file_type == XvcFileType::File
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

                    match ContentDigest::from_path(&path, algorithm, text_or_binary.as_inner()) {
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

    let path_prefix = current_dir.strip_prefix(&xvc_root.absolute_path())?;

    let rows = matches
        .into_iter()
        .filter_map(|pm| match ListRow::new(xvc_root, path_prefix, pm) {
            Ok(lr) => Some(lr),
            Err(e) => {
                error!(output_snd, "{}", e);
                None
            }
        })
        .collect();

    let list_rows = ListRows::new(opts.format.unwrap(), opts.sort_criteria.unwrap(), rows);
    output!(output_snd, "{}", list_rows.build_table(!opts.no_summary));
    Ok(())
}
