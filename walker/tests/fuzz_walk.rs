//! Property-based (fuzz) tests for the directory walkers.
//!
//! Each case generates a random directory tree with random `.gitignore` files, then
//! asserts that:
//!
//! - `walk_serial` and `walk_parallel` return exactly the same set of files,
//! - that set is exactly what the ignore rules dictate: a file is returned if and only
//!   if neither the file nor any of its ancestor directories is ignored.
//!
//! Nested ignore files only get patterns containing a slash (anchored to the ignore
//! file's directory). Unanchored patterns in nested ignore files apply to the whole
//! tree from the moment the walker loads them, so with those the result would depend
//! on traversal timing and could not be compared deterministically.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use proptest::prelude::*;
use xvc_walker::{
    IgnoreRules, MatchResult, WalkOptions, build_ignore_patterns, walk_parallel, walk_serial,
};

const COMPONENTS: &[&str] = &[
    "alpha", "beta", "gamma", "delta", "data", "logs", "cache", "deep",
];
const EXTENSIONS: &[&str] = &["bin", "txt", "log"];

fn component() -> impl Strategy<Value = String> {
    prop::sample::select(COMPONENTS).prop_map(String::from)
}

fn extension() -> impl Strategy<Value = String> {
    prop::sample::select(EXTENSIONS).prop_map(String::from)
}

fn file_name() -> impl Strategy<Value = String> {
    (component(), extension()).prop_map(|(name, ext)| format!("{name}.{ext}"))
}

/// A relative file path: 0 to 3 directory components plus a file name.
fn rel_file() -> impl Strategy<Value = PathBuf> {
    (prop::collection::vec(component(), 0..4), file_name()).prop_map(|(dirs, file)| {
        let mut path = PathBuf::new();
        for dir in dirs {
            path.push(dir);
        }
        path.push(file);
        path
    })
}

/// A pattern line for the root ignore file. May be anchored or unanchored, ignore or
/// whitelist.
fn root_ignore_line() -> impl Strategy<Value = String> {
    prop_oneof![
        file_name(),
        extension().prop_map(|e| format!("*.{e}")),
        component().prop_map(|c| format!("{c}/")),
        (component(), file_name()).prop_map(|(c, f)| format!("{c}/{f}")),
        (component(), extension()).prop_map(|(c, e)| format!("{c}/*.{e}")),
        component().prop_map(|c| format!("**/{c}")),
        file_name().prop_map(|f| format!("!{f}")),
        extension().prop_map(|e| format!("!*.{e}")),
    ]
}

/// A pattern line for a nested ignore file. Always contains a non-final slash so the
/// pattern stays anchored to the directory of the ignore file.
fn nested_ignore_line() -> impl Strategy<Value = String> {
    prop_oneof![
        (component(), file_name()).prop_map(|(c, f)| format!("{c}/{f}")),
        (component(), extension()).prop_map(|(c, e)| format!("{c}/*.{e}")),
        (component(), file_name()).prop_map(|(c, f)| format!("!{c}/{f}")),
    ]
}

/// Creates the files under `root`, skipping paths that conflict with already created
/// ones (a file cannot also be a directory). Returns the relative paths actually
/// created.
fn create_tree(root: &Path, files: &BTreeSet<PathBuf>) -> BTreeSet<PathBuf> {
    let mut created_files = BTreeSet::new();
    let mut created_dirs = BTreeSet::new();

    'candidate: for rel in files {
        if created_dirs.contains(rel) {
            continue;
        }
        let mut prefix = PathBuf::new();
        let mut parents = Vec::new();
        for comp in rel.parent().unwrap_or(Path::new("")).components() {
            prefix.push(comp);
            if created_files.contains(&prefix) {
                continue 'candidate;
            }
            parents.push(prefix.clone());
        }
        fs::create_dir_all(root.join(rel.parent().unwrap_or(Path::new("")))).unwrap();
        fs::write(root.join(rel), b"content").unwrap();
        created_dirs.extend(parents);
        created_files.insert(rel.clone());
    }

    created_files
}

/// Computes the set of files the walkers must return: a file is included iff neither
/// itself nor any ancestor directory is ignored by `rules`.
fn expected_files(
    root: &Path,
    rules: &IgnoreRules,
    files: &BTreeSet<PathBuf>,
) -> BTreeSet<PathBuf> {
    files
        .iter()
        .filter(|rel| {
            let mut prefix = PathBuf::new();
            for comp in rel.components() {
                prefix.push(comp);
                if matches!(rules.check(&root.join(&prefix)), MatchResult::Ignore) {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect()
}

fn files_relative_to(root: &Path, paths: impl IntoIterator<Item = PathBuf>) -> BTreeSet<PathBuf> {
    paths
        .into_iter()
        .map(|p| p.strip_prefix(root).unwrap().to_path_buf())
        .collect()
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]

    #[test]
    fn walkers_agree_and_respect_ignore_rules(
        files in prop::collection::btree_set(rel_file(), 1..20),
        root_ignore in prop::collection::vec(root_ignore_line(), 0..8),
        nested_ignore in prop::option::of((any::<prop::sample::Index>(), prop::collection::vec(nested_ignore_line(), 1..4))),
    ) {
        let root = xvc_test_helper::random_temp_dir(Some("xvc-walker-fuzz"));
        fs::create_dir_all(&root).unwrap();

        let mut all_files = create_tree(&root, &files);

        // Write the root ignore file.
        fs::write(root.join(".gitignore"), root_ignore.join("\n")).unwrap();
        all_files.insert(PathBuf::from(".gitignore"));

        // Optionally write a nested ignore file into one of the created directories.
        if let Some((index, lines)) = &nested_ignore {
            let dirs: BTreeSet<PathBuf> = all_files
                .iter()
                .filter_map(|f| f.parent())
                .filter(|p| !p.as_os_str().is_empty())
                .map(Path::to_path_buf)
                .collect();
            if !dirs.is_empty() {
                let dirs: Vec<&PathBuf> = dirs.iter().collect();
                let dir = dirs[index.index(dirs.len())];
                let rel = dir.join(".gitignore");
                if !all_files.contains(&rel) {
                    fs::write(root.join(&rel), lines.join("\n")).unwrap();
                    all_files.insert(rel);
                }
            }
        }

        let walk_options = WalkOptions {
            ignore_filename: Some(".gitignore".to_string()),
            include_dirs: true,
        };

        // Serial walk.
        let (output_snd, _output_rcv) = crossbeam_channel::unbounded();
        let (serial_paths, _) = walk_serial(&output_snd, "", &root, &walk_options).unwrap();
        let serial_files = files_relative_to(
            &root,
            serial_paths
                .into_iter()
                .filter(|pm| pm.metadata.is_file())
                .map(|pm| pm.path),
        );

        // Parallel walk.
        let shared_rules = Arc::new(RwLock::new(IgnoreRules::from_global_patterns(
            &root,
            Some(".gitignore"),
            "",
        )));
        let (path_snd, path_rcv) = crossbeam_channel::unbounded();
        walk_parallel(shared_rules, &root, walk_options, path_snd).unwrap();
        let parallel_files = files_relative_to(
            &root,
            path_rcv
                .into_iter()
                .filter_map(|res| res.ok())
                .filter(|pm| pm.metadata.is_file())
                .map(|pm| pm.path),
        );

        // The oracle: full ignore rules for the tree, applied to every path and its
        // ancestors.
        let rules = build_ignore_patterns("", &root, ".gitignore").unwrap();
        let expected = expected_files(&root, &rules, &all_files);

        fs::remove_dir_all(&root).ok();

        prop_assert_eq!(
            &serial_files,
            &parallel_files,
            "serial and parallel walks disagree"
        );
        prop_assert_eq!(
            &serial_files,
            &expected,
            "walk result disagrees with ignore rules"
        );
    }
}
