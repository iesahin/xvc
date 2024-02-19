use std::{
    fs,
    path::{Path, PathBuf},
    process::exit,
};

use anyhow::anyhow;

use regex::Regex;

use xvc::error::Result;
use xvc_logging::{info, watch};
use xvc_test_helper::{make_symlink, random_temp_dir, test_logging};

use fs_extra::{self, dir::CopyOptions};

const DOCS_SOURCE_DIR: &str = "../book/src/";
const DOCS_TARGET_DIR: &str = "docs/";
const TEMPLATE_DIR: &str = "templates/";

fn book_dirs_and_filters() -> Vec<(String, String)> {
    let trycmd_tests = if let Ok(trycmd_tests) = std::env::var("XVC_TRYCMD_TESTS") {
        trycmd_tests.to_lowercase()
    } else {
        "intro,start,how-to,storage,file,pipeline,core".to_owned()
    };

    let mut book_dirs_and_filters = vec![];

    if trycmd_tests.contains("intro") {
        book_dirs_and_filters.push(("intro".to_owned(), r".*".to_owned()));
    }
    if trycmd_tests.contains("start") {
        book_dirs_and_filters.push(("start".to_owned(), r".*".to_owned()));
    }

    let mut howto_regex = ".*".to_owned();
    if trycmd_tests.contains("how-to") || trycmd_tests.contains("howto") {
        watch!(std::env::var("XVC_TRYCMD_HOWTO_REGEX"));
        if let Ok(regex) = std::env::var("XVC_TRYCMD_HOWTO_REGEX") {
            howto_regex.push_str(&regex);
            howto_regex.push_str(".*");
        }
        book_dirs_and_filters.push(("how-to".to_owned(), howto_regex));
    }

    if trycmd_tests.contains("core") {
        book_dirs_and_filters.push(("ref".to_owned(), r"^xvc-[^psf].*".to_owned()))
    }

    if trycmd_tests.contains("file") {
        book_dirs_and_filters.push(("ref".to_owned(), r"xvc-file.*".to_owned()));
    }

    if trycmd_tests.contains("pipeline") {
        book_dirs_and_filters.push(("ref".to_owned(), r"xvc-pipeline.*".to_owned()));
    }

    if trycmd_tests.contains("storage") {
        if let Ok(storages) = std::env::var("XVC_TRYCMD_STORAGE_TESTS") {
            let storage_elements = storages.split(',').collect::<Vec<_>>();
            storage_elements.iter().for_each(|s| {
                book_dirs_and_filters.push(("ref".to_owned(), format!("xvc-storage.*{}.*", s)));
            })
        } else {
            book_dirs_and_filters.push(("ref".to_owned(), r"xvc-storage-.*".to_owned()));
        }
    }
    book_dirs_and_filters
}

fn link_to_docs() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    let docs_source_root = Path::new(DOCS_SOURCE_DIR);

    let templates_target_root = random_temp_dir(Some("xvc-trycmd"));

    watch!(TEMPLATE_DIR);
    watch!(templates_target_root);
    watch!(Path::new(TEMPLATE_DIR).exists());

    fs_extra::dir::copy(
        Path::new(TEMPLATE_DIR),
        &templates_target_root,
        &CopyOptions {
            copy_inside: true,
            ..Default::default()
        },
    )
    .map_err(|e| anyhow::format_err!("Directory Error: {}", e))?;

    let docs_target_root = Path::new(DOCS_TARGET_DIR);
    watch!(docs_target_root);
    remove_all_symlinks_under(docs_target_root)?;
    let book_dirs_and_filters = book_dirs_and_filters();

    watch!(book_dirs_and_filters);

    for (doc_section_dir_name, filter_regex) in book_dirs_and_filters {
        let name_filter = Regex::new(&filter_regex).unwrap();

        let doc_source_paths =
            filter_paths_under(docs_source_root, &doc_section_dir_name, name_filter);

        for doc_source_path in doc_source_paths {
            watch!(doc_source_path);
            make_markdown_link(&doc_source_path, docs_target_root)?;

            make_input_dir_link(&doc_source_path, docs_target_root, &templates_target_root)?;

            make_output_dir_link(&doc_source_path, docs_target_root, &templates_target_root)?;
        }
    }

    Ok(())
}
fn markdown_link_name(doc_source_path: &Path) -> PathBuf {
    watch!(doc_source_path);
    doc_source_path
        .to_string_lossy()
        .strip_prefix(DOCS_SOURCE_DIR)
        .unwrap_or(&doc_source_path.to_string_lossy())
        .to_owned()
        .replace('/', "-")
        .into()
}

fn input_dir_name(doc_source_path: &Path) -> PathBuf {
    markdown_link_name(doc_source_path)
        .file_stem()
        .map(|s| {
            let mut s = s.to_string_lossy().to_string();
            s.push_str(".in");
            PathBuf::from(s)
        })
        .unwrap()
}

fn output_dir_path(doc_source_path: &Path) -> PathBuf {
    markdown_link_name(doc_source_path)
        .file_stem()
        .map(|s| {
            let mut s = s.to_string_lossy().to_string();
            s.push_str(".out");
            PathBuf::from(s)
        })
        .unwrap()
}

fn make_markdown_link(doc_source_path: &Path, docs_target_dir: &Path) -> Result<PathBuf> {
    let target = docs_target_dir.join(markdown_link_name(doc_source_path));
    watch!(&target);
    let source = Path::new("..").join(doc_source_path);
    watch!(&source);
    if target.exists() && target.read_link().unwrap() == source {
        fs::remove_file(&target).unwrap_or_else(|e| {
            info!("Failed to remove file: {}", e);
            exit(1);
        });
    }
    make_symlink(&source, &target)?;
    Ok(target)
}

fn make_input_dir_link(
    doc_source_path: &Path,
    docs_target_dir: &Path,
    templates_root: &Path,
) -> Result<PathBuf> {
    let dirname = input_dir_name(doc_source_path);
    let source = templates_root.join(&dirname);
    watch!(&source);
    if !source.exists() {
        fs::create_dir_all(&source)?;
    }
    let target = docs_target_dir.join(&dirname);
    watch!(&target);
    if target.exists() && target.read_link().unwrap() == source {
        fs::remove_file(&target).unwrap_or_else(|e| {
            info!("Failed to remove file: {}", e);
            exit(1);
        });
    }
    make_symlink(&source, &target)?;
    Ok(source)
}

fn make_output_dir_link(
    doc_source_path: &Path,
    docs_target_dir: &Path,
    templates_root: &Path,
) -> Result<PathBuf> {
    let dirname = output_dir_path(doc_source_path);
    watch!(&dirname);
    let source = templates_root.join(&dirname);
    watch!(source);
    if !source.exists() {
        let target = docs_target_dir.join(&dirname);
        watch!(target.exists());
        if target.exists() {
            watch!(target.read_link().unwrap());
            if target.read_link().unwrap() == source {
                fs::remove_file(&target).unwrap_or_else(|e| {
                    info!("Failed to remove file: {}", e);
                    exit(1);
                });
            }
        }
        make_symlink(&source, target)?;
    }
    Ok(source)
}

fn filter_paths_under(
    test_doc_source_root: &Path,
    doc_section_dir_name: &str,
    name_filter: Regex,
) -> Vec<PathBuf> {
    let test_doc_source_paths: Vec<PathBuf> =
        jwalk::WalkDir::new(test_doc_source_root.join(doc_section_dir_name))
            .into_iter()
            .filter_map(|f| {
                watch!(f);
                if let Ok(f) = f {
                    let file_name = f.file_name().to_string_lossy();
                    watch!(file_name);
                    if f.metadata().unwrap().is_file() && name_filter.is_match(&file_name) {
                        watch!((&name_filter, "matched", &file_name));
                        Some(f.path())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
    test_doc_source_paths
}

fn remove_all_symlinks_under(dir: &Path) -> Result<()> {
    //Remove all symlinks to create new ones
    jwalk::WalkDir::new(dir).into_iter().for_each(|f| {
        watch!(f);
        if let Ok(f) = f {
            let path = f.path();
            if path.is_symlink() {
                fs::remove_file(&path).unwrap();
            }
        }
    });
    Ok(())
}

#[test]
#[cfg(target_os = "macos")]
fn z_doc_tests() -> Result<()> {
    use std::time::Duration;

    link_to_docs()?;
    watch!("Linking done");

    let xvc_th = escargot::CargoBuild::new()
        .bin("xvc-test-helper")
        .current_release()
        .current_target()
        .manifest_path("../test_helper/Cargo.toml")
        .run()
        .map_err(|e| anyhow!("Failed to build xvc-test-helper: {e:?}"))?;

    watch!("Built xvc-test-helper");

    let path_to_xvc_test_helper = xvc_th.path().to_path_buf();
    watch!(path_to_xvc_test_helper);
    assert!(path_to_xvc_test_helper.exists());

    let timeout = if let Ok(secs) = std::env::var("XVC_TRYCMD_DURATION") {
        Duration::from_secs(secs.parse::<u64>().unwrap())
    } else if std::option_env!("CI").is_some() {
        Duration::from_secs(90)
    } else {
        Duration::from_secs(30)
    };

    trycmd::TestCases::new()
        // .register_bin("xvc", &path_to_xvc_bin)
        .register_bin("xvc-test-helper", &path_to_xvc_test_helper)
        .register_bin("git", which::which("git"))
        .register_bin("echo", which::which("echo"))
        .register_bin("cat", which::which("cat"))
        .register_bin("ls", which::which("ls"))
        .register_bin("rm", which::which("rm"))
        .register_bin("perl", which::which("perl"))
        .register_bin("tree", which::which("tree"))
        .register_bin("zsh", which::which("zsh"))
        .register_bin("dot", which::which("dot"))
        .register_bin("unzip", which::which("unzip"))
        .register_bin("python3", which::which("python3"))
        .register_bin("dvc", which::which("dvc"))
        .register_bin("hyperfine", which::which("hyperfine"))
        .case("docs/*.md")
        .timeout(timeout)
        // We skip this for the time being.
        .skip("docs/start/ml.md");

    Ok(())
}
