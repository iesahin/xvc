use crate::error::{Error, Result};
use clap::Parser;
use crossbeam_channel::Sender;
use path_absolutize::Absolutize;
use relative_path::RelativePath;
use xvc_logging::XvcOutputLine;

#[derive(Debug, Parser)]
#[clap(name = "alias")]
/// Print aliases in the common format to be added to `.zsh_aliases`, `.bash_aliases` or
/// `.profile`.
///
/// You can use
///
/// ```
/// $(xvc aliases)
/// ```
///
/// in these files, or edit after redirecting like
///
/// ```
/// $ xvc alias >> ~/.zsh_aliases
///
/// ```
///
pub struct AliasesCLI {}

/// Standard Xvc command aliases for longer commands.
pub const XVC_ALIASES: &'static str = r#"
alias xls='xvc file list'
alias pvc='xvc pipeline'
alias fvc='xvc file'
alias xvcf='xvc file'
alias xvcft='xvc file track'
alias xvcfl='xvc file list'
alias xvcfs='xvc file send'
alias xvcfb='xvc file bring'
alias xvcfh='xvc file hash'
alias xvcfc='xvc file checkout'
alias xvcp='xvc pipeline'
alias xvcpr='xvc pipeline run'
alias xvcps='xvc pipeline step'
alias xvcpsn='xvc pipeline step new'
alias xvcpsd='xvc pipeline step dependency'
alias xvcpso='xvc pipeline step output'
alias xvcpi='xvc pipeline import'
alias xvcpe='xvc pipeline export'
alias xvcpl='xvc pipeline list'
alias xvcpn='xvc pipeline new'
alias xvcpu='xvc pipeline update'
alias xvcpd='xvc pipeline dag'
alias xvcs='xvc storage'
alias xvcsn='xvc storage new'
alias xvcsl='xvc storage list'
alias xvcsr='xvc storage remove'
"#;

/// Print out aliases for long commands.
/// These can be sourced in `~/.zsh_aliases`, `~/.bash_aliases` etc. like `$(xvc aliases)`.
pub fn run(output_snd: Sender<XvcOutputLine>, _opts: AliasesCLI) -> Result<()> {
    output_snd
        .send(XvcOutputLine::Output(XVC_ALIASES.to_string()))
        .unwrap();

    Ok(())
}
