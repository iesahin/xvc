use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::anyhow;

use regex::Regex;

use xvc::error::Result;
use xvc_logging::watch;
use xvc_test_helper::{make_symlink, random_temp_dir, test_logging};

use fs_extra::{self, dir::CopyOptions};

const DOC_TEST_DIR: &str = "docs/";

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

    if trycmd_tests.contains("storage") {
        book_dirs_and_filters.push(("ref".to_owned(), r"xvc-storage.*".to_owned()));
    }
    if trycmd_tests.contains("file") {
        book_dirs_and_filters.push(("ref".to_owned(), r"xvc-file.*".to_owned()));
    }

    if trycmd_tests.contains("pipeline") {
        book_dirs_and_filters.push(("ref".to_owned(), r"xvc-pipeline.*".to_owned()));
    }

    if trycmd_tests.contains("core") {
        book_dirs_and_filters.push(("ref".to_owned(), r"^xvc-[^psf].*".to_owned()))
    }

    if trycmd_tests.contains("intro") {
        book_dirs_and_filters.push(("intro".to_owned(), r".*".to_owned()));
    }
    if trycmd_tests.contains("start") {
        book_dirs_and_filters.push(("start".to_owned(), r".*".to_owned()));
    }

    if trycmd_tests.contains("storage") {
        if let Ok(storages) = std::env::var("XVC_TRYCMD_STORAGE_TESTS") {
            let storage_elements = storages.split(',').collect::<Vec<_>>();
            storage_elements.iter().for_each(|s| {
                book_dirs_and_filters.push(("ref".to_owned(), format!("xvc-storage-{}.*", s)));
            })
        } else {
            book_dirs_and_filters.push(("ref".to_owned(), r"xvc-storage-.*".to_owned()));
        }
    }
    if trycmd_tests.contains("file") {
        book_dirs_and_filters.push(("ref".to_owned(), r"xvc-file.*".to_owned()));
    }

    if trycmd_tests.contains("pipeline") {
        book_dirs_and_filters.push(("ref".to_owned(), r"xvc-pipeline.*".to_owned()));
    }

    if trycmd_tests.contains("core") {
        book_dirs_and_filters.push(("ref".to_owned(), r"^xvc-[^psf].*".to_owned()))
    }

    book_dirs_and_filters
}

fn link_to_docs() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    let book_base = Path::new("../book/src/");

    let template_dir_root = Path::new("templates");

    // This is a directory that we create to keep testing artifacts outside the code
    // It has the same structure with the docs, but for each doc.md file, a doc.in/ and doc.out/
    // directory is created and these are linked from the running directory.
    let test_collections_dir = random_temp_dir(Some("xvc-trycmd"));

    println!(
        "Documentation Test Directory: {}",
        test_collections_dir.to_string_lossy()
    );

    fs::create_dir_all(&test_collections_dir)?;

    //Remove all symlinks to create new ones
    let doc_dir = Path::new(DOC_TEST_DIR);
    watch!(doc_dir);
    jwalk::WalkDir::new(doc_dir).into_iter().for_each(|f| {
        watch!(f);
        if let Ok(f) = f {
            if f.metadata().unwrap().is_symlink() {
                fs::remove_file(f.path()).unwrap();
            }
        }
    });

    let book_dirs_and_filters = book_dirs_and_filters();

    watch!(book_dirs_and_filters);

    for (dir, filter_regex) in book_dirs_and_filters {
        let dir = &dir.as_str();
        let test_collection_dir = test_collections_dir.join(dir);
        let name_filter = Regex::new(&filter_regex).unwrap();

        let book_dir = book_base.join(dir);
        assert!(book_dir.exists(), "{:?} doesn't exist", &book_dir);
        let book_paths: Vec<PathBuf> = jwalk::WalkDir::new(book_base.join(dir))
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

        watch!(test_collection_dir);
        fs::create_dir_all(&test_collection_dir)?;
        for p in book_paths {
            let basename: PathBuf = p.file_name().unwrap().into();
            let symlink_path = doc_dir.join(dir).join(&basename);

            watch!(symlink_path);
            make_symlink(Path::new("../..").join(p), &symlink_path)?;

            // If we have a template input directory in `templates/`, we copy it.
            // Otherwise create a new blank directory as cwd.
            let stem = basename.file_stem().unwrap().to_string_lossy();
            let in_dir_name = format!("{stem}.in");
            let in_dir = test_collection_dir.join(&in_dir_name);
            let cwd = env::current_dir()?;
            let input_template_dir = cwd.join(template_dir_root.join(&in_dir_name));
            if input_template_dir.exists() {
                println!("Copying template dir: {input_template_dir:?} to {in_dir:?}");
                fs_extra::dir::copy(
                    &input_template_dir,
                    &test_collection_dir,
                    &CopyOptions::default(),
                )
                .map_err(|e| anyhow!("FS Extra Error: {e:?}"))?;
            } else {
                fs::create_dir(&in_dir)?;
            }

            // Link to the directory TMPDIR we just created above.
            // This is to renew test input for each run.
            let in_dir_symlink = doc_dir.join(dir).join(&in_dir_name);
            if in_dir_symlink.is_symlink() {
                fs::remove_file(&in_dir_symlink)?;
            }
            make_symlink(&in_dir, &in_dir_symlink)?;

            // Create output dir if only template dir exists
            let out_dir_name = format!("{stem}.out");
            let output_template_dir = template_dir_root.join(&out_dir_name);
            if output_template_dir.exists() {
                let out_dir = test_collection_dir.join(&out_dir_name);
                let out_dir_symlink = doc_dir.join(dir).join(&out_dir_name);
                if out_dir_symlink.is_symlink() {
                    fs::remove_file(&out_dir_symlink)?;
                }
                make_symlink(&out_dir, &out_dir_symlink)?;
                watch!(&out_dir);
            }
        }
    }

    Ok(())
}

#[test]
#[cfg(target_os = "macos")]
fn z_doc_tests() -> Result<()> {
    use std::time::Duration;

    link_to_docs()?;

    let xvc_th = escargot::CargoBuild::new()
        .bin("xvc-test-helper")
        .current_release()
        .current_target()
        .manifest_path("../test_helper/Cargo.toml")
        .run()
        .map_err(|e| anyhow!("Failed to build xvc-test-helper: {e:?}"))?;

    let path_to_xvc_test_helper = xvc_th.path().to_path_buf();
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
        .case("docs/*/*.md")
        .timeout(timeout)
        // We skip this for the time being.
        .skip("docs/start/ml.md");

    Ok(())
}
