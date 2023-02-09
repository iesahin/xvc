use crate::common::{
    cache_paths_for_xvc_paths, filter_targets_from_store, load_targets_from_store,
};
use crate::Result;
use chrono::DateTime;
use clap::Parser;
use itertools::Itertools;
use parse_size::parse_size;
use xvc_core::{XvcCachePath, XvcRoot};
use xvc_logging::{output, warn, XvcOutputSender};
use xvc_storage::storage::get_storage_record;
use xvc_storage::{StorageIdentifier, XvcStorageOperations};

/// Remove files from Xvc cache or storage
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(rename_all = "kebab-case", author, version)]
pub struct RemoveCLI {
    /// Remove files from cache
    #[arg(long)]
    from_cache: bool,

    /// Remove files from storage
    #[arg(long)]
    from_storage: Option<StorageIdentifier>,

    /// Remove all versions of the file
    #[arg(long)]
    all_versions: bool,

    /// Remove only the specified version of the file
    ///
    /// Versions are specified like b3-123-456-789abcd where b3 is the hash algorithm prefix and the rest is a (at least
    /// 3 digit) prefix of the content hash. Prefix must be unique. If the prefix is not unique, the command will fail.
    /// Dashes are optional.
    #[arg(long, conflicts_with_all = ["all_versions", "before", "after", "larger_than", "smaller_than"])]
    only_version: Option<String>,

    /// Remove the targets even if they are used by other targets (via deduplication)
    #[arg(long)]
    force: bool,

    /// Files/directories to remove
    #[arg()]
    targets: Vec<String>,
}

pub(crate) fn cmd_remove(
    output_snd: &XvcOutputSender,
    xvc_root: &XvcRoot,
    opts: RemoveCLI,
) -> Result<()> {
    if !opts.from_cache && opts.from_storage.is_none() {
        return Err(anyhow::anyhow!(
            "At least one of --from-cache or --from-storage must be specified"
        )
        .into());
    }

    if opts.all_versions && opts.only_version.is_some() {
        return Err(
            anyhow::anyhow!("Cannot specify both --all-versions and --only-version").into(),
        );
    }

    let current_dir = xvc_root.config().current_dir()?;

    let all_paths = xvc_root.load_store()?;
    let all_content_digests = xvc_root.load_store()?;
    let remove_targets =
        filter_targets_from_store(xvc_root, &all_paths, current_dir, &Some(opts.targets))?;

    let all_cache_paths = cache_paths_for_xvc_paths(output_snd, &all_paths, &all_content_digests)?;

    let cache_paths_for_targets = all_cache_paths.subset(remove_targets.keys().copied())?;
    let candidate_paths = if opts.all_versions {
        cache_paths_for_targets
            .iter()
            .map(|(xe, vec_cp)| vec_cp.iter().map(|cp| (xe, cp)).collect::<Vec<_>>())
            .flatten()
            .collect::<Vec<_>>()
    } else {
        if let Some(version) = opts.only_version {
            let version_cmp_str = version.replace("-", "");
            let version_cmp = |v| v.replace("/", "").starts_with(version_cmp_str);
            let paths = cache_paths_for_targets
                .iter()
                .map(|(xe, vec_cp)| vec_cp.iter().filter(version_cmp).collect())
                .flatten()
                .collect::<Vec<_>>();

            if paths.len() > 1 {
                return Err(anyhow::anyhow!(
                    "Version prefix is not unique:\n{}",
                    paths.iter().map(|(_, xcp)| xcp.to_string()).join("\n")
                )
                .into());
            } else {
                paths
            }
        } else {
            remove_targets
                .iter()
                .map(|(xe, xp)| {
                    (
                        xe,
                        XvcCachePath::new(xp, all_content_digests.get(xe).unwrap()),
                    )
                })
                .collect::<Vec<_>>()
        }
    };

    let mut entities_for_cache_path: HashMap<XvcCachePath, HashSet<XvcEntity>> = HashMap::new();

    for (xe, cache_paths) in all_cache_paths.iter() {
        for cp in cache_paths {
            if !entities_for_cache_path.contains_key(cp) {
                entities_for_cache_path.insert(cp.clone(), HashSet::new());
            }
            let entity_set = entities_for_cache_path.get_mut(cp).unwrap();
            entity_set.insert(*xe);
        }
    }

    let mut deletable_paths = Vec::<XvcCachePath>::new();
    // Report the differences if found
    let removable_entities = remove_targets.keys().collect();
    for (xe, cp) in candidate_paths {
        let entities_pointing_to_cp =
            HashSet::from_iter(entities_for_cache_path[&cp].iter().copied());

        let mut deletable = true;
        entities_pointing_to_cp
            .difference(&removable_entities)
            .for_each(|other_xe| {
                let this_xp = all_paths.get(&xe).unwrap();
                let other_xp = all_paths.get(other_xe).unwrap();
                if opts.force {
                    warn!(
                        output_snd,
                        "Deleting {} (for {}) even though it's also used by {}!",
                        cp,
                        this_xp,
                        other_xp
                    );
                } else {
                    output!(
                        output_snd,
                        "Not deleting {} (for {}) because it's also used by {}",
                        cp,
                        this_xp,
                        other_xp
                    );
                    deletable = false;
                }
            });

        if deletable {
            deletable_paths.push(cp);
        }
    }

    if opts.from_cache {
        deletable_paths
            .iter()
            .for_each(|xcp| xcp.remove(output_snd, xvc_root)?);
    }

    if let Some(storage) = opts.from_storage {
        let storage = get_storage_record(output_snd, xvc_root, &storage)?;
        storage.delete(output_snd, xvc_root, deletable_paths.as_slice())?;
    }

    Ok(())
}
