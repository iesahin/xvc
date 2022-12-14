use std::{
    env,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use jwalk;
use trycmd;
use which;
use xvc::error::Result;
use xvc_test_helper::{make_symlink, random_temp_dir, test_logging};
use xvc_tests::watch;

use fs_extra::{self, dir::CopyOptions};

const DOC_TEST_DIR: &str = "docs/";

fn link_to_docs() -> Result<()> {
    test_logging(log::LevelFilter::Trace);
    let book_base = Path::new("../book/src/");
    let book_dirs = vec!["ref", "start", "how-to"];
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

    let doc_dir = Path::new(DOC_TEST_DIR);

    for dir in book_dirs {
        let test_collection_dir = test_collections_dir.join(dir);

        let book_dir = book_base.join(dir);
        assert!(book_dir.exists(), "{:?} doesn't exist", &book_dir);
        let book_paths: Vec<PathBuf> = jwalk::WalkDir::new(book_base.join(dir))
            .into_iter()
            .filter_map(|f| {
                if let Ok(f) = f {
                    if f.metadata().unwrap().is_file()
                        && f.path().extension() == Some(OsStr::new("md"))
                    {
                        Some(f.path())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        fs::create_dir_all(&test_collection_dir)?;

        for p in book_paths {
            let basename: PathBuf = p.file_name().unwrap().into();
            let symlink_path = doc_dir.join(dir).join(&basename);
            if symlink_path.is_symlink() {
                fs::remove_file(&symlink_path)?;
            }
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
            watch!(&in_dir_symlink);
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
            }
        }
    }

    Ok(())
}

#[test]
fn doc_tests() -> Result<()> {
    link_to_docs()?;

    trycmd::TestCases::new()
        .register_bin("git", which::which("git")?)
        .register_bin("echo", Path::new("/bin/echo"))
        .register_bin("cat", Path::new("/bin/cat"))
        .register_bin("ls", Path::new("/bin/ls"))
        .register_bin("rm", Path::new("/bin/rm"))
        .case("docs/*/*.md")
        // We skip this for the time being.
        .skip("docs/start/ml.md");

    Ok(())
}
