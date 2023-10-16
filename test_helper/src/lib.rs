//! Helper functions to create random temporary directories, random binary or text files, and set logging in unit/integration
//! tests.
//! The directories may be initialized as Git or Xvc repositories.
#![warn(missing_docs)]
#![forbid(unsafe_code)]
use log::LevelFilter;
use rand::distributions::Alphanumeric;
use rand::rngs::StdRng;
use rand::Rng;
use rand::RngCore;
use rand::SeedableRng;
use std::cmp;
use std::env;
use std::fs::OpenOptions;

use std::{
    fs::{self, File},
    process::Command,
};
use std::{
    io::{self, BufWriter, Write},
    path::{Path, PathBuf},
};

use anyhow::Result;
use xvc_logging::{setup_logging, watch};

#[cfg(unix)]
use std::os::unix::fs as unix_fs;
#[cfg(windows)]
use std::os::windows::fs as windows_fs;

/// Turn on logging for testing purposes.
/// Testing always send traces to `$TMPDIR/xvc.log`.
/// The `level` here determines whether these are sent to `stdout`.
pub fn test_logging(level: LevelFilter) {
    setup_logging(Some(level), Some(level));
}

/// Generates a random name with `prefix` and a random number generated from `seed`.
/// If `seed` is `None`, a random number `from_entropy` is used.
pub fn random_dir_name(prefix: &str, seed: Option<u64>) -> String {
    let mut rng = if let Some(seed) = seed {
        rand::rngs::StdRng::seed_from_u64(seed)
    } else {
        rand::rngs::StdRng::from_entropy()
    };

    let rand: u32 = rng.next_u32();
    format!("{}-{}", prefix, rand)
}

/// Return name of a random directory under $TMPDIR.
/// It doesn't create the directory, just returns the path.
pub fn random_temp_dir(prefix: Option<&str>) -> PathBuf {
    let mut temp_dir = env::temp_dir();
    loop {
        let cand = temp_dir.join(Path::new(&random_dir_name(
            prefix.unwrap_or("xvc-repo"),
            None,
        )));
        if !cand.exists() {
            temp_dir = cand;
            break;
        }
    }

    temp_dir
}

/// Return a temp directory created with a seed.
/// If `seed` is `None`, it creates a random directory name.
/// This function doesn't create the directory.
pub fn seeded_temp_dir(prefix: &str, seed: Option<u64>) -> PathBuf {
    let temp_dir = env::temp_dir();
    temp_dir.join(Path::new(&random_dir_name(prefix, seed)))
}

/// Create a random named temp directory under $TMPDIR
pub fn create_temp_dir() -> PathBuf {
    let temp_dir = random_temp_dir(None);

    fs::create_dir_all(&temp_dir).expect("Cannot create directory.");
    temp_dir
}

/// Create a temporary dir under $TMPDIR and cd to it
pub fn run_in_temp_dir() -> PathBuf {
    let temp_dir = create_temp_dir();
    watch!(temp_dir);
    env::set_current_dir(&temp_dir).expect("Cannot change directory");
    temp_dir
}

/// Create an empty temporary Git repository without XVC_DIR
pub fn run_in_temp_git_dir() -> PathBuf {
    let temp_dir = run_in_temp_dir();
    let output = Command::new("git")
        .arg("init")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
    watch!(output);
    temp_dir
}

/// Create a random directory and run `git init` in it.
pub fn temp_git_dir() -> PathBuf {
    let temp_dir = create_temp_dir();
    watch!(temp_dir);
    Command::new("git")
        .arg("-C")
        .arg(temp_dir.as_os_str())
        .arg("init")
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));
    temp_dir
}

/// Generate a random binary file
pub fn generate_random_file(filename: &Path, size: usize, seed: Option<u64>) {
    let f = OpenOptions::new()
        .create(true)
        .write(true)
        .open(filename)
        .unwrap();
    let mut writer = BufWriter::new(f);

    let mut rng: StdRng = seed
        .map(StdRng::seed_from_u64)
        .unwrap_or_else(StdRng::from_entropy);
    let mut buffer = [0u8; 1024];
    let mut remaining_size = size;

    while remaining_size > 0 {
        let to_write = cmp::min(remaining_size, buffer.len());
        let buffer = &mut buffer[0..to_write];
        rng.fill(buffer);
        writer.write_all(buffer).unwrap();

        remaining_size -= to_write;
    }
}

/// Creates a file filled with byte
pub fn generate_filled_file(filename: &Path, size: usize, byte: u8) {
    let f = File::create(filename).unwrap();
    let mut writer = BufWriter::new(f);
    let buffer = [byte; 1024];
    let mut remaining_size = size;
    while remaining_size > 0 {
        let to_write = cmp::min(remaining_size, buffer.len());
        let buffer = &buffer[0..to_write];
        writer.write_all(buffer).unwrap();
        remaining_size -= to_write;
    }
}

/// Generate a random text file composed of alphanumerics
pub fn generate_random_text_file(filename: &Path, num_lines: usize) {
    let mut f = File::create(filename).unwrap();
    let rng = rand::thread_rng();
    let line_length = 100;
    for _ in 0..num_lines {
        let line: String = rng
            .clone()
            .sample_iter(&Alphanumeric)
            .take(line_length)
            .map(char::from)
            .collect();
        writeln!(f, "{}\n", line).expect("Could not write to file.");
    }
}

/// Build a directory tree containing `n_dirs` under `root`.
/// Each of these directories contain `n_files_per_dir` random binary files.
pub fn create_directory_tree(
    root: &Path,
    n_dirs: usize,
    n_files_per_dir: usize,
    min_size: usize,
    seed: Option<u64>,
) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::<PathBuf>::with_capacity(n_dirs * n_files_per_dir);
    let dirs: Vec<String> = (1..=n_dirs).map(|i| format!("dir-{:04}", i)).collect();
    let files: Vec<(String, usize)> = (1..=n_files_per_dir)
        .map(|i| (format!("file-{:04}.bin", i), min_size + i + 1000))
        .collect();
    for dir in dirs {
        std::fs::create_dir_all(root.join(Path::new(&dir)))?;
        paths.extend(files.iter().map(|(name, size)| {
            let filename = PathBuf::from(&format!("{}/{}/{}", root.to_string_lossy(), dir, name));
            generate_random_file(&filename, *size, seed);
            filename
        }));
    }
    Ok(paths)
}

#[cfg(unix)]
/// Creates a symlink from target to original
pub fn make_symlink<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> io::Result<()> {
    unix_fs::symlink(original, link)
}

#[cfg(windows)]
/// Creates a file symlink from target to original
pub fn make_symlink<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> io::Result<()> {
    windows_fs::symlink_file(original, link)
}
