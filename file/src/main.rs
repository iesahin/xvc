use clap::Parser;

fn main() -> xvc_file::error::Result<()> {
    let cli_opts = xvc_file::XvcFileCLI::parse();
    xvc_file::dispatch(cli_opts)
}
