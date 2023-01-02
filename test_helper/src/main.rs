use std::path::PathBuf;

use clap::Parser;
use xvc_test_helper::{
    create_directory_tree, create_temp_dir, generate_filled_file, generate_random_file,
    generate_random_text_file, random_dir_name, random_temp_dir, seeded_temp_dir, temp_git_dir,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Parser)]
#[command(version, author)]
struct XvcTestHelperCLI {
    #[clap(subcommand)]
    subcommand: XvcTestHelperSubcommandCLI,
}

#[derive(Parser)]
enum XvcTestHelperSubcommandCLI {
    /// Create a directory in $TMPDIR
    CreateTempDir,
    /// Create a directory in $TMPDIR and init Git in it
    CreateTempGitDir,
    /// Create a directory tree
    CreateDirectoryTree {
        /// The root directory to create the tree in
        #[clap(short, long)]
        root: Option<PathBuf>,
        /// The number of directories to create
        #[clap(short, long, default_value = "1")]
        directories: usize,
        /// The number of files to create
        #[clap(short, long, default_value = "1")]
        files: usize,
    },

    RandomDirName {
        /// The seed to supply to the random number generator
        #[clap(short, long)]
        seed: Option<u64>,
        #[clap(short, long)]
        prefix: Option<String>,
    },

    RandomTempDir {
        #[clap(short, long)]
        prefix: Option<String>,
    },

    GenerateRandomFile {
        /// The size of the file to generate
        #[clap(short, long, default_value = "1024")]
        size: usize,
        #[clap(short, long)]
        filename: String,
    },

    GenerateFilledFile {
        /// The size of the file to generate
        #[clap(short, long, default_value = "1024")]
        size: usize,
        /// The byte to fill the file with
        #[clap(short, long, default_value = "0")]
        byte: u8,
        #[clap(short, long)]
        filename: String,
    },

    GenerateRandomTextFile {
        /// The number of lines to generate
        #[clap(short, long, default_value = "1")]
        lines: usize,
        #[clap(short, long)]
        filename: String,
    },
}

fn main() -> Result<()> {
    let cli_opts = XvcTestHelperCLI::parse();
    match cli_opts.subcommand {
        XvcTestHelperSubcommandCLI::CreateTempDir => {
            println!("{}", create_temp_dir().to_string_lossy());
        }
        XvcTestHelperSubcommandCLI::CreateTempGitDir => {
            println!("{}", temp_git_dir().to_string_lossy())
        }
        XvcTestHelperSubcommandCLI::CreateDirectoryTree {
            root,
            directories,
            files,
        } => {
            let root = root.unwrap_or_else(|| std::env::current_dir().unwrap());
            create_directory_tree(&root, directories, files);
        }
        XvcTestHelperSubcommandCLI::RandomDirName { seed, prefix } => {
            let name = random_dir_name(
                &prefix.unwrap_or_else(|| "xvc-test-helper".to_string()),
                seed,
            );
            println!("{}", name);
        }
        XvcTestHelperSubcommandCLI::RandomTempDir { prefix } => {
            let dir = random_temp_dir(prefix.as_deref());
            println!("{}", dir.to_string_lossy());
        }
        XvcTestHelperSubcommandCLI::GenerateRandomFile { size, filename } => {
            let path = PathBuf::from(filename);
            generate_random_file(&path, size);
        }
        XvcTestHelperSubcommandCLI::GenerateFilledFile {
            size,
            byte,
            filename,
        } => {
            let path = PathBuf::from(filename);
            generate_filled_file(&path, size, byte);
        }
        XvcTestHelperSubcommandCLI::GenerateRandomTextFile { lines, filename } => {
            let path = PathBuf::from(filename);
            generate_random_text_file(&path, lines);
        }
    }

    Ok(())
}