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
    let test_doc_source_root = Path::new("../book/src/");

    let test_doc_working_dir_templates_root = Path::new("templates");

    // This is a directory that we create to keep testing artifacts outside the code
    // It has the same structure with the docs, but for each doc.md file, a doc.in/ and doc.out/
    // directory is created and these are linked from the running directory.
    let temporary_test_root = random_temp_dir(Some("xvc-trycmd"));

    println!(
        "Documentation Test Directory: {}",
        temporary_test_root.to_string_lossy()
    );

    fs::create_dir_all(&temporary_test_root)?;

    let test_doc_dir = Path::new(DOC_TEST_DIR);
    remove_all_symlinks_under(test_doc_dir)?;

    let book_dirs_and_filters = book_dirs_and_filters();

    watch!(book_dirs_and_filters);

    for (doc_section_dir_name, filter_regex) in book_dirs_and_filters {
        // ref, intro, start, how-to
        let doc_section_dir = temporary_test_root.join(&doc_section_dir_name);
        if !doc_section_dir.exists() {
            fs::create_dir_all(&doc_section_dir)?;
        }

        let name_filter = Regex::new(&filter_regex).unwrap();

        let test_doc_source_paths =
            filter_paths_under(test_doc_source_root, &doc_section_dir_name, name_filter);

        let book_dir = test_doc_source_root.join(&doc_section_dir_name);
        assert!(book_dir.exists(), "{:?} doesn't exist", &book_dir);

        for test_doc_source_path in test_doc_source_paths {
            let test_doc_source_filename =
                make_document_link(test_doc_source_path, test_doc_dir, &doc_section_dir_name)?;

            let stem = test_doc_source_filename
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string();

            make_template_input_dir(
                &stem,
                test_doc_working_dir_templates_root,
                &doc_section_dir,
                test_doc_dir,
            )?;

            make_template_output_dir(
                &stem,
                test_doc_working_dir_templates_root,
                &doc_section_dir,
                test_doc_dir,
            )?;
        }
    }

    Ok(())
}

fn make_template_output_dir(
    stem: &str,
    test_doc_working_dir_templates_root: &Path,
    doc_section_dir: &PathBuf,
    test_doc_dir: &Path,
) -> Result<()> {
    let out_dir_name = format!("{stem}.out");
    let output_template_dir = test_doc_working_dir_templates_root.join(&out_dir_name);
    if output_template_dir.exists() {
        let out_dir = doc_section_dir.join(&out_dir_name);
        let out_dir_symlink = test_doc_dir.join(doc_section_dir).join(&out_dir_name);
        if out_dir_symlink.is_symlink() {
            fs::remove_file(&out_dir_symlink)?;
        }
        watch!(&out_dir);
        watch!(&out_dir_symlink);
        make_symlink(&out_dir, &out_dir_symlink)?;
        watch!(&out_dir);
    }
    Ok(())
}

fn make_template_input_dir(
    stem: &str,
    test_doc_working_dir_templates_root: &Path,
    doc_section_dir: &PathBuf,
    test_doc_dir: &Path,
) -> Result<()> {
    let template_dir_name = format!("{stem}.in");
    watch!(template_dir_name);
    let target_template_dir = doc_section_dir.join(&template_dir_name);
    watch!(target_template_dir);
    let cwd = env::current_dir()?;
    watch!(cwd);
    let input_template_dir = cwd.join(test_doc_working_dir_templates_root.join(&template_dir_name));
    watch!(input_template_dir);
    if input_template_dir.exists() {
        watch!(input_template_dir);
        watch!(target_template_dir);
        watch!(doc_section_dir);
        fs_extra::dir::copy(
            &input_template_dir,
            doc_section_dir,
            &CopyOptions::default(),
        )
        .map_err(|e| anyhow!("FS Extra Error: {e:?}"))?;
    } else {
        watch!((&input_template_dir, "doesn't exist"));
        fs::create_dir(&target_template_dir)?;
    }
    watch!(&test_doc_dir);
    let in_dir_symlink = test_doc_dir
        .join(doc_section_dir.clone())
        .join(&template_dir_name);
    if in_dir_symlink.is_symlink() {
        fs::remove_file(&in_dir_symlink)?;
    }
    make_symlink(&target_template_dir, &in_dir_symlink)?;
    Ok(())
}

fn make_document_link(
    test_doc_source_path: PathBuf,
    test_doc_dir: &Path,
    doc_section_dir_name: &String,
) -> Result<PathBuf> {
    watch!(test_doc_source_path);
    let test_doc_source_filename: PathBuf = test_doc_source_path.file_name().unwrap().into();
    watch!(test_doc_source_filename);
    let test_doc_symlink = test_doc_dir
        .join(doc_section_dir_name.clone())
        .join(&test_doc_source_filename);
    watch!(test_doc_symlink);
    let test_doc_symlink_orig = Path::new("../..").join(test_doc_source_path);
    watch!(&test_doc_symlink_orig);
    watch!(test_doc_symlink.exists());
    make_symlink(&test_doc_symlink_orig, &test_doc_symlink)?;
    Ok(test_doc_source_filename)
}

fn filter_paths_under(
    test_doc_source_root: &Path,
    doc_section_dir_name: &String,
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
