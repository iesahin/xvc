//! Data structures and functions specific to `xvc file list`
//!
//! - [ListCLI] defines the command line options
//! - [cmd_list]  is the entry point to run the command
use crate::common::calc_digest;
use crate::common::compare::PathComparisonParams;
use crate::Result;
use chrono;
use clap::Parser;
use crossbeam_channel::Sender;
use log::warn;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::SystemTime;
use strum_macros::{Display as EnumDisplay, EnumString};
use xvc_config::{conf, FromConfigKey, UpdateFromXvcConfig, XvcConfig};
use xvc_core::{CacheType, ContentDigest, TextOrBinary, XvcPath, XvcRoot};
use xvc_ecs::{HStore, XvcEntity};
use xvc_logging::{watch, XvcOutputLine};
use xvc_walker::{directory_list, IgnoreRules, PathMetadata, WalkOptions};

#[derive(Debug, Copy, Clone, EnumString, EnumDisplay, PartialEq, Eq)]
#[strum(serialize_all = "kebab-case")]
enum ListColumn {
    CacheType,
    CacheStatus,
    Size,
    Timestamp,
    Name,
    ContentHash,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ListColumns {
    columns: Vec<ListColumn>,
}

impl FromStr for ListColumns {
    type Err = crate::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut columns = Vec::new();
        for lc in s.split(',') {
            columns.push(ListColumn::from_str(lc)?);
        }
        Ok(Self { columns })
    }
}

conf!(ListColumns, "file.list.columns");

#[derive(Debug, Copy, Clone, EnumString, EnumDisplay, PartialEq, Eq)]
#[strum(serialize_all = "kebab-case")]
enum ListSortCriteria {
    None,
    NameAsc,
    NameDesc,
    SizeAsc,
    SizeDesc,
    TimestampAsc,
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

    cache_status: String,
    cache_type: String,
    name: String,

    recorded_content_digest_str: String,
    recorded_size: u64,
    recorded_size_str: String,
    recorded_timestamp: SystemTime,
    recorded_timestamp_str: String,
}

impl ListRow {
    fn new(
        xvc_root: &XvcRoot,
        path_prefix: &Path,
        pcp: &PathComparisonParams,
        path_match: PathMatch,
    ) -> Self {
        let found_actual_found_record =
            |xvc_entity, path_metadata: PathMetadata, actual_digest: Option<ContentDigest>| {
                let cache_type = format_cache_type(
                    &pcp.cache_type_store
                        .get(&xvc_entity)
                        .cloned()
                        .unwrap_or_default(),
                );

                let actual_content_digest_str = match actual_digest {
                    Some(digest) => format!("{}", digest),
                    None => "".to_string(),
                };
                let actual_size = path_metadata.metadata.len();
                let actual_size_str = format_size(actual_size);
                let actual_timestamp = path_metadata.metadata.modified().unwrap();
                let actual_timestamp_str = format_timestamp(actual_timestamp);
                let path = path_metadata
                    .path
                    .strip_prefix(path_prefix)
                    .unwrap_or_else(|_| &path_metadata.path);
                let name = path.to_string_lossy().to_string();

                let recorded_metadata = pcp
                    .xvc_metadata_store
                    .get(&xvc_entity)
                    .expect(&format!("Cannot find XvcMetadata Record for {xvc_entity}"));
                let recorded_content_digest_str = "".to_string();
                let recorded_size = recorded_metadata.size.unwrap();
                let recorded_size_str = format_size(recorded_size);
                let recorded_timestamp = recorded_metadata.modified.unwrap();
                let recorded_timestamp_str = format_timestamp(recorded_timestamp);

                let cache_status = if recorded_timestamp < actual_timestamp {
                    "<".to_string()
                } else if recorded_timestamp > actual_timestamp {
                    ">".to_string()
                } else {
                    "=".to_string()
                };

                ListRow {
                    cache_type,
                    cache_status,
                    actual_size,
                    actual_size_str,
                    recorded_size,
                    recorded_size_str,
                    actual_timestamp,
                    actual_timestamp_str,
                    recorded_timestamp,
                    recorded_timestamp_str,
                    recorded_content_digest_str,
                    actual_content_digest_str,
                    name,
                }
            };

        let found_actual_missing_record =
            |path_metadata: PathMetadata, actual_digest: Option<ContentDigest>| {
                let (cache_type, cache_status) = if path_metadata.metadata.file_type().is_dir() {
                    ("D".to_string(), " ".to_string())
                } else {
                    ("M".to_string(), "<".to_string())
                };

                let actual_content_digest_str = match actual_digest {
                    Some(digest) => format!("{}", digest),
                    None => "".to_string(),
                };
                let actual_size = path_metadata.metadata.len();
                let actual_size_str = format_size(actual_size);
                let actual_timestamp = path_metadata.metadata.modified().unwrap();
                let actual_timestamp_str = format_timestamp(actual_timestamp);

                let path = path_metadata
                    .path
                    .strip_prefix(path_prefix)
                    .unwrap_or_else(|_| &path_metadata.path);
                let name = path.to_string_lossy().to_string();

                let recorded_size = 0;
                let recorded_size_str = "".to_string();
                let recorded_timestamp = SystemTime::UNIX_EPOCH;
                let recorded_timestamp_str = "".to_string();
                let recorded_content_digest_str = "".to_string();

                ListRow {
                    cache_type,
                    cache_status,
                    actual_size,
                    actual_size_str,
                    recorded_size,
                    recorded_size_str,
                    actual_timestamp,
                    actual_timestamp_str,
                    recorded_timestamp,
                    recorded_timestamp_str,
                    recorded_content_digest_str,
                    actual_content_digest_str,
                    name,
                }
            };

        let missing_actual_found_record = |xvc_entity| {
            let cache_type = format_cache_type(pcp.cache_type_store.get(&xvc_entity).unwrap());
            let cache_status = ">".to_string();

            let actual_content_digest_str = "".to_string();
            let actual_size = 0;
            let actual_size_str = "".to_string();
            let actual_timestamp = SystemTime::UNIX_EPOCH;
            let actual_timestamp_str = "".to_string();

            let xvc_path = pcp.xvc_path_store.get(&xvc_entity).unwrap();
            let path = xvc_path.to_absolute_path(xvc_root);
            let path = path.strip_prefix(path_prefix).unwrap_or_else(|_| &path);
            let name = path.to_string_lossy().to_string();

            let recorded_metadata = pcp.xvc_metadata_store.get(&xvc_entity).unwrap();
            let recorded_content_digest =
                pcp.content_digest_store.get(&xvc_entity).unwrap().clone();
            let recorded_content_digest_str = (recorded_content_digest.0)
                .map(|d| d.hex_str())
                .unwrap_or_else(|| "".to_string());
            let recorded_size = recorded_metadata.size.unwrap();
            let recorded_size_str = format_size(recorded_size);
            let recorded_timestamp = recorded_metadata
                .modified
                .unwrap_or_else(|| SystemTime::UNIX_EPOCH);
            let recorded_timestamp_str = format_timestamp(recorded_timestamp);

            ListRow {
                cache_type,
                cache_status,
                actual_size,
                actual_size_str,
                recorded_size,
                recorded_size_str,
                actual_timestamp,
                actual_timestamp_str,
                recorded_timestamp,
                recorded_timestamp_str,
                recorded_content_digest_str,
                actual_content_digest_str,
                name,
            }
        };

        match (path_match.actual_path, path_match.xvc_entity) {
            (None, None) => {
                panic!("Neither record, nor path exists, how this could be?");
            }
            (Some(pm), None) => found_actual_missing_record(pm, path_match.actual_digest),
            (None, Some(xvc_entity)) => missing_actual_found_record(xvc_entity),
            (Some(pm), Some(xvc_entity)) => {
                found_actual_found_record(xvc_entity, pm, path_match.actual_digest)
            }
        }
    }
}

fn format_cache_type(cache_type: &CacheType) -> String {
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

fn format_size(size: u64) -> String {
    if size < 1024 * 1024 {
        format!("{:10}", size)
    } else if size < 1024 * 1024 * 1024 {
        format!("{:>4}MB{}", size / 1024 / 1024, size % 1024)
    } else if size < 1024 * 1024 * 1024 * 1024 {
        format!("{:>4}GB{}", size / 1024 / 1024 / 1024, size % 1024)
    } else {
        format!("{:>4}TB{}", size / 1024 / 1024 / 1024 / 1024, size % 1024)
    }
}

#[derive(Debug, Clone)]
struct PathMatch {
    actual_path: Option<PathMetadata>,
    xvc_entity: Option<XvcEntity>,
    actual_digest: Option<ContentDigest>,
}

#[derive(Debug, Clone)]
struct ListRows {
    rows: Vec<ListRow>,
    columns: ListColumns,
    sort_criteria: ListSortCriteria,
    actual: bool,
}

impl ListRows {
    pub fn new(
        columns: ListColumns,
        sort_criteria: ListSortCriteria,
        rows: Vec<ListRow>,
        actual: bool,
    ) -> Self {
        Self {
            rows,
            columns,
            sort_criteria,
            actual,
        }
    }
}

const DIGEST_CHARS: usize = 8;

impl Display for ListRows {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut rows = self.rows.clone();
        let columns = self.columns.clone();
        let sort_criteria = self.sort_criteria;
        if sort_criteria != ListSortCriteria::None {
            rows.sort_by(|a, b| {
                let a_cmp = match sort_criteria {
                    ListSortCriteria::NameAsc => a.name.cmp(&b.name),
                    ListSortCriteria::NameDesc => b.name.cmp(&a.name),
                    ListSortCriteria::SizeAsc => a.recorded_size.cmp(&b.actual_size),
                    ListSortCriteria::SizeDesc => b.recorded_size.cmp(&a.actual_size),
                    ListSortCriteria::TimestampAsc => a.recorded_timestamp.cmp(&b.actual_timestamp),
                    ListSortCriteria::TimestampDesc => {
                        b.recorded_timestamp.cmp(&a.actual_timestamp)
                    }
                    _ => unreachable!(),
                };
                a_cmp
            });
        }
        for row in rows {
            for column in &columns.columns {
                match column {
                    ListColumn::CacheType => write!(f, "{}", row.cache_type)?,
                    ListColumn::CacheStatus => write!(f, "{}", row.cache_status)?,
                    ListColumn::Size => {
                        if self.actual {
                            write!(f, "\t{:>20}", row.actual_size_str)?
                        } else {
                            write!(f, "\t{:>20}", row.recorded_size_str)?
                        }
                    }
                    ListColumn::Timestamp => {
                        if self.actual {
                            write!(f, "\t{:<20}", row.actual_timestamp_str)?
                        } else {
                            write!(f, "\t{:<20}", row.recorded_timestamp_str)?
                        }
                    }
                    ListColumn::Name => write!(f, "\t{}", row.name)?,
                    ListColumn::ContentHash => {
                        if self.actual {
                            let actual_digest_str =
                                if row.actual_content_digest_str.len() > DIGEST_CHARS {
                                    &row.actual_content_digest_str[..DIGEST_CHARS]
                                } else {
                                    &row.actual_content_digest_str
                                };
                            write!(f, "\t{:<8}", actual_digest_str)?
                        } else {
                            let recorded_digest_str =
                                if row.recorded_content_digest_str.len() > DIGEST_CHARS {
                                    &row.recorded_content_digest_str[..DIGEST_CHARS]
                                } else {
                                    &row.recorded_content_digest_str
                                };

                            write!(f, "\t{:<8}", recorded_digest_str)?
                        }
                    }
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(about = "List files tracked with XVC ", rename_all = "kebab-case")]

pub struct ListCLI {
    /// Columns to show
    #[arg(long, short = 'c')]
    columns: Option<ListColumns>,
    /// Sort by column
    #[arg(long, short = 's', alias = "sort")]
    sort_criteria: Option<ListSortCriteria>,
    /// Show directory contents
    #[arg(long, short = 'R')]
    recursive: bool,
    /// Show actual data instead of recorded
    #[arg(long, short)]
    actual: bool,
    /// Files/directories to list
    #[arg()]
    target: Option<PathBuf>,
}

impl UpdateFromXvcConfig for ListCLI {
    fn update_from_conf(
        self,
        conf: &xvc_config::XvcConfig,
    ) -> xvc_config::error::Result<Box<Self>> {
        let columns = self.columns.unwrap_or_else(|| ListColumns::from_conf(conf));
        let sort_criteria = self
            .sort_criteria
            .unwrap_or_else(|| ListSortCriteria::from_conf(conf));
        let current_dir = conf.current_dir()?;
        Ok(Box::new(Self {
            columns: Some(columns),
            sort_criteria: Some(sort_criteria),
            recursive: self.recursive,
            actual: self.actual,
            target: Some(self.target.unwrap_or_else(|| current_dir.to_path_buf())),
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
    let calculate_content_hash = opts.actual
        && opts
            .columns
            .as_ref()
            .map(|cols| cols.columns.contains(&ListColumn::ContentHash))
            .unwrap_or_else(|| false);

    watch!(calculate_content_hash);

    let target = opts.target.unwrap();

    watch!(target);

    let current_dir = conf.current_dir()?;

    let xvc_target = XvcPath::new(xvc_root, current_dir, &target)?;

    watch!(xvc_target);

    let mut found_paths = Vec::<(XvcPath, PathMetadata)>::new();

    let pcp: PathComparisonParams = PathComparisonParams::init(xvc_root)?;
    let mut recorded_paths = HStore::<XvcPath>::new();
    // TODO: The following can be done in parallel
    watch!(opts.recursive);
    if target.is_dir() {
        let mut res_paths = Vec::<_>::new();
        if opts.recursive {
            xvc_walker::walk_serial(
                IgnoreRules::empty(xvc_root),
                &target,
                &WalkOptions {
                    ignore_filename: None,
                    include_dirs: true,
                },
                &mut res_paths,
            )?;
        } else {
            res_paths.extend(directory_list(&target)?);
        }

        watch!(&res_paths.len());

        for res_path in res_paths {
            match res_path {
                Ok(pm) => {
                    let xvc_path = XvcPath::new(xvc_root, current_dir, &pm.path)?;
                    found_paths.push((xvc_path, pm));
                }
                Err(e) => {
                    warn!("{}", e);
                }
            }
        }
    } else {
        if target.is_file() {
            let pm = PathMetadata {
                path: target.clone(),
                metadata: target.metadata().unwrap(),
            };
            let xvc_path = XvcPath::new(xvc_root, current_dir, &pm.path)?;
            found_paths.push((xvc_path, pm));
        }
    }

    watch!(found_paths.len());

    // TODO: Support for symlinks etc

    if target.is_dir() {
        let abs_target = xvc_target.to_absolute_path(xvc_root);
        for (xvc_entity, xvc_path) in pcp.xvc_path_store.iter() {
            if opts.recursive && xvc_path.to_absolute_path(xvc_root).starts_with(&abs_target) {
                recorded_paths.insert(*xvc_entity, xvc_path.clone());
            }
            if !opts.recursive {
                if let Some(parent) = xvc_path.to_absolute_path(xvc_root).parent() {
                    if parent.as_os_str() == abs_target.as_os_str() {
                        recorded_paths.insert(*xvc_entity, xvc_path.clone());
                    }
                }
            }
        }
    }

    if target.is_file() {
        if let Some(xvc_entity) = pcp.xvc_path_imap.get(&xvc_target) {
            recorded_paths.insert(*xvc_entity, xvc_target);
        }
    }

    // Now match actual and recorded paths

    let mut matches = Vec::<PathMatch>::new();
    let mut found_entities = Vec::<XvcEntity>::new();

    for (xvc_path, pm) in found_paths.drain(..) {
        if let Some(xvc_entity) = pcp.xvc_path_imap.get(&xvc_path) {
            let digest = if calculate_content_hash {
                let text_or_binary = pcp
                    .text_or_binary_store
                    .get(xvc_entity)
                    .map(|d| d.as_inner())
                    .unwrap_or_else(|| TextOrBinary::Auto);
                Some(calc_digest(&pm.path, pcp.algorithm, text_or_binary)?.into())
            } else {
                None
            };

            matches.push(PathMatch {
                xvc_entity: Some(xvc_entity.clone()),
                actual_path: Some(pm),
                actual_digest: digest,
            });
            found_entities.push(xvc_entity.clone());
        } else {
            let digest = if calculate_content_hash && (pm.path.as_path().is_file()) {
                Some(calc_digest(&pm.path, pcp.algorithm, TextOrBinary::Auto)?.into())
            } else {
                None
            };
            matches.push(PathMatch {
                xvc_entity: None,
                actual_path: Some(pm),
                actual_digest: digest,
            });
        }
    }

    for (xvc_entity, _) in recorded_paths.iter() {
        if !found_entities.contains(xvc_entity) {
            matches.push(PathMatch {
                xvc_entity: Some(*xvc_entity),
                actual_path: None,
                actual_digest: None,
            });
        }
    }

    let mut rows = Vec::<ListRow>::new();
    for m in matches.drain(..) {
        rows.push(ListRow::new(xvc_root, current_dir, &pcp, m));
    }

    output_snd.send(
        format!(
            "{}",
            ListRows::new(
                opts.columns.unwrap(),
                opts.sort_criteria.unwrap(),
                rows,
                opts.actual
            )
        )
        .into(),
    )?;
    Ok(())
}
