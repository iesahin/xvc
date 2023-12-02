//! The home of `xvc file remove` command.
//!
//! [`RemoveCLI`] defines the options of the command, and [`cmd_remove`] is the entry point.
use std::collections::{HashMap, HashSet};

use crate::common::{cache_paths_for_xvc_paths, filter_targets_from_store};
use crate::Result;

use clap::Parser;
use itertools::Itertools;

use xvc_core::types::xvcdigest::DIGEST_LENGTH;
use xvc_core::{XvcCachePath, XvcRoot};
use xvc_ecs::XvcEntity;
use xvc_logging::{output, uwr, warn, watch, XvcOutputSender};
use xvc_storage::storage::get_storage_record;
use xvc_storage::{StorageIdentifier, XvcStorageOperations};

/// Remove files from Xvc cache or storage
#[derive(Debug, Clone, PartialEq, Eq, Parser)]
#[command(rename_all = "kebab-case", author, version)]
pub struct RemoveCLI {
    /// Remove files from cache
    #[arg(long, required_unless_present = "from_storage")]
    from_cache: bool,

    /// Remove files from storage
    #[arg(long, required_unless_present = "from_cache")]
    from_storage: Option<StorageIdentifier>,

    /// Remove all versions of the file
    #[arg(long)]
    all_versions: bool,

    /// Remove only the specified version of the file
    ///
    /// Versions are specified with the content hash 123-456-789abcd.
    /// Dashes are optional.
    /// Prefix must be unique. If the prefix is not unique, the command will fail.
    #[arg(long, conflicts_with = "all_versions")]
    only_version: Option<String>,

    /// Remove the targets even if they are used by other targets (via deduplication)
    #[arg(long)]
    force: bool,

    /// Files/directories to remove
    #[arg()]
    targets: Vec<String>,
}

/// Removes a file from XVC cache or storage
pub fn cmd_remove(output_snd: &XvcOutputSender, xvc_root: &XvcRoot, opts: RemoveCLI) -> Result<()> {
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
        // Return all cache paths used by the targets
        cache_paths_for_targets
            .iter()
            .flat_map(|(xe, vec_cp)| {
                vec_cp
                    .iter()
                    .map(|cp| (*xe, cp.clone()))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    } else if let Some(version) = opts.only_version {
        // Return only the cache paths that match the version prefix
        let version_cmp_str = version.replace('-', "");
        watch!(version_cmp_str);
        let version_cmp = |v: &&XvcCachePath| {
            let digest_str = v.digest_string(DIGEST_LENGTH).replace('-', "");
            watch!(digest_str);
            // We skip the first two characters because they are the hash algorithm identifier
            digest_str[2..].starts_with(&version_cmp_str)
        };
        let paths = cache_paths_for_targets
            .iter()
            .filter_map(|(xe, vec_cp)| {
                let possible_paths = vec_cp
                    .iter()
                    .filter(version_cmp)
                    .cloned()
                    .collect::<Vec<XvcCachePath>>();
                if !possible_paths.is_empty() {
                    Some((*xe, possible_paths))
                } else {
                    None
                }
            })
            .fold(
                Vec::<(XvcEntity, XvcCachePath)>::new(),
                |mut acc, (xe, vec_cp)| {
                    vec_cp.into_iter().for_each(|xcp| acc.push((xe, xcp)));
                    watch!(acc);
                    acc
                },
            );

        watch!(paths);

        if paths.len() > 1 {
            return Err(anyhow::anyhow!(
                "Version prefix is not unique:\n{}",
                paths
                    .iter()
                    .map(|(_, xcp)| xcp.digest_string(DIGEST_LENGTH))
                    .join("\n")
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
                    *xe,
                    XvcCachePath::new(xp, all_content_digests.get(xe).unwrap()).unwrap(),
                )
            })
            .collect::<Vec<(XvcEntity, XvcCachePath)>>()
    };

    watch!(candidate_paths);

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
    let removable_entities: HashSet<XvcEntity> = remove_targets.keys().copied().collect();
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

    // We sort the paths to have a stable output.
    deletable_paths.sort_unstable();

    if opts.from_cache {
        deletable_paths.iter().for_each(|xcp| {
            watch!(xcp);
            uwr!(xcp.remove(output_snd, xvc_root), output_snd)
        });
    }

    if let Some(storage) = opts.from_storage {
        let storage = get_storage_record(output_snd, xvc_root, &storage)?;
        storage.delete(output_snd, xvc_root, deletable_paths.as_slice())?;
    }

    Ok(())
}
