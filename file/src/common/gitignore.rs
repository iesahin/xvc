use chrono::Utc;
use crossbeam_channel::Sender;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::thread::JoinHandle;
use xvc_core::util::git::build_gitignore;

use crate::Result;
use xvc_core::{XvcPath, XvcRoot};
use xvc_logging::{debug, error, info, uwr, watch, XvcOutputLine};
use xvc_walker::{check_ignore, AbsolutePath, IgnoreRules, MatchResult};

pub enum IgnoreOperation {
    IgnoreDir { dir: XvcPath },
    IgnoreFile { file: XvcPath },
}

pub type IgnoreOp = Option<IgnoreOperation>;

pub fn make_ignore_handler(
    output_snd: &Sender<XvcOutputLine>,
    xvc_root: &XvcRoot,
) -> Result<(Sender<IgnoreOp>, JoinHandle<()>)> {
    let (sender, receiver) = crossbeam_channel::unbounded();
    let output_snd = output_snd.clone();
    let xvc_root = xvc_root.absolute_path().clone();

    let handle = std::thread::spawn(move || {
        let mut ignore_dirs = Vec::<XvcPath>::new();
        let mut ignore_files = Vec::<XvcPath>::new();

        let gitignore = build_gitignore(&xvc_root).unwrap();
        for op in receiver {
            if let Some(op) = op {
                match op {
                    IgnoreOperation::IgnoreDir { dir } => {
                        let path = dir.to_absolute_path(&xvc_root).to_path_buf();

                        if !ignore_dirs.contains(&dir)
                            && matches!(check_ignore(&gitignore, &path), MatchResult::NoMatch)
                        {
                            ignore_dirs.push(dir);
                        }
                    }
                    IgnoreOperation::IgnoreFile { file } => {
                        let path = file.to_absolute_path(&xvc_root).to_path_buf();
                        if !ignore_files.contains(&file)
                            && matches!(check_ignore(&gitignore, &path), MatchResult::NoMatch)
                        {
                            ignore_files.push(file);
                        }
                    }
                }
            } else {
                // We quit the loop when we get None
                break;
            }
        }
        debug!(output_snd, "Writing directories to .gitignore");
        uwr!(
            update_dir_gitignores(&xvc_root, &xvc_root, &gitignore, &ignore_dirs),
            output_snd
        );
        // Load again to get ignored directories
        let gitignore = build_gitignore(&xvc_root).unwrap();
        debug!(output_snd, "Writing files to .gitignore");
        uwr!(
            update_file_gitignores(&xvc_root, &xvc_root, &gitignore, &ignore_files),
            output_snd
        );
    });

    Ok((sender, handle))
}

/// Write file and directory names to .gitignore found in the same dir
///
/// If `current_ignore` already ignores a file, it's not added separately.
/// If the user chooses to ignore a files manually by general rules, files are not added here.
///
pub fn update_dir_gitignores(
    xvc_root: &AbsolutePath,
    current_dir: &AbsolutePath,
    current_gitignore: &IgnoreRules,
    dirs: &[XvcPath],
) -> Result<()> {
    // Check if dirs are already ignored
    let dir_map: HashMap<XvcPath, PathBuf> = dirs
        .iter()
        .filter_map(|dir| {
            let abs_path = if dir.ends_with("/") {
                current_dir.join(dir.to_string())
            } else {
                current_dir.join(format!("{}/", dir.to_string()))
            };

            let ignore_res = check_ignore(current_gitignore, &abs_path);

            match ignore_res {
                MatchResult::Ignore => {
                    info!("Path is already gitignored: {}", abs_path.to_string_lossy());
                    None
                }
                MatchResult::NoMatch => {
                    Some((dir.clone(),
                          abs_path.parent()
                            .map(|p| p.join(".gitignore"))
                            .unwrap_or_else(|| PathBuf::from(".gitignore"))))
                }
                MatchResult::Whitelist => {
                    error!("Path is whitelisted in Git. Please remove/modify the whitelisting rule: {}",
                        abs_path.to_string_lossy());
                    None
                }
            }}).collect();

    watch!(dir_map);

    // Check if files are already ignored
    let mut changes = HashMap::<PathBuf, Vec<String>>::new();

    for (d, gi) in dir_map {
        if !changes.contains_key(&gi) {
            changes.insert(gi.clone(), Vec::<String>::new());
        }

        let path_v = changes.get_mut(&gi).unwrap();
        path_v.push(
            d.file_name()
                .map(|d| format!("/{}/", d))
                .unwrap_or_else(|| "## Path Contains final ..".to_string()),
        );
    }

    for (gitignore_file, values) in changes {
        let append_str = format!(
            "### Following {} lines are added by xvc on {}\n{}",
            values.len(),
            Utc::now().to_rfc2822(),
            values.join("\n")
        );
        let gitignore_path = xvc_root.join(gitignore_file);

        let mut file_o = OpenOptions::new()
            .create(true)
            .append(true)
            .open(gitignore_path)?;

        writeln!(file_o, "{}", append_str)?;
    }

    Ok(())
}

pub fn update_file_gitignores(
    xvc_root: &AbsolutePath,
    current_dir: &AbsolutePath,
    current_gitignore: &IgnoreRules,
    files: &[PathBuf],
) -> Result<()> {
    // Check if files are already ignored
    let file_map: HashMap<PathBuf, PathBuf> = files
        .iter()
        .filter_map(|f| {
                    let abs_path = current_dir.join(f);

            match check_ignore(current_gitignore, &abs_path) {
                MatchResult::NoMatch => {

                    Some((f.clone(),
                          f.parent()
                            .map(|p| p.join(".gitignore"))
                            .unwrap_or_else(|| PathBuf::from(".gitignore"))))
                }
                MatchResult::Ignore => {
                    info!("Already gitignored: {}", &abs_path.to_string_lossy());
                    None
                }
                MatchResult::Whitelist => {
                    error!("Path is whitelisted in Gitignore, please modify/remove the whitelisting rule: {}", &abs_path.to_string_lossy());
                None
            }}
            })
        .collect();

    watch!(file_map);

    let mut changes = HashMap::<PathBuf, Vec<String>>::new();

    for (f, gi) in file_map {
        if !changes.contains_key(&gi) {
            changes.insert(gi.clone(), Vec::<String>::new());
        }

        let path_v = changes.get_mut(&gi).unwrap();
        path_v.push(
            f.file_name()
                .map(|f| format!("/{}", f.to_string_lossy()))
                .unwrap_or_else(|| "## Path Contains final ..".to_string()),
        );
    }

    for (gitignore_file, values) in changes {
        let append_str = format!(
            "### Following {} lines are added by xvc on {}\n{}",
            values.len(),
            Utc::now().to_rfc2822(),
            values.join("\n")
        );
        let gitignore_path = xvc_root.join(gitignore_file);

        let mut file_o = OpenOptions::new()
            .create(true)
            .append(true)
            .open(gitignore_path)?;

        writeln!(file_o, "{}", append_str)?;
    }

    Ok(())
}
