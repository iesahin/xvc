# xvc aliases

## Synopsis

```console
$ xvc aliases --help
Print command aliases to be sourced in shell files

Usage: xvc aliases

Options:
  -h, --help  Print help information

```


## Examples

You can include aliases in interactive shells. 

```console,ignore
$ . $(xvc aliases)
$ pvc --help
Pipeline management commands

Usage: xvc pipeline [OPTIONS] <COMMAND>

Commands:
  new     Add a new pipeline
  update  Rename, change dir or set a pipeline default
  delete  Delete a pipeline
  run     Run a pipeline
  list    List all pipelines
  dag     Generate mermaid diagram for the pipeline
  export  Export the pipeline to a YAML, TOML or JSON file
  import  Import the pipeline from a file
  step    Step management commands
  help    Print this message or the help of the given subcommand(s)

Options:
  -n, --name <NAME>  Name of the pipeline this command applies to
  -h, --help         Print help information
```

If you add the above line to your `.bashrc` or `.zshrc`, these aliases will always be available.

You can get a list of aliases. 

```console
$ xvc aliases

alias xls='xvc file list'
alias pvc='xvc pipeline'
alias fvc='xvc file'
alias xvcf='xvc file'
alias xvcft='xvc file track'
alias xvcfl='xvc file list'
alias xvcfs='xvc file send'
alias xvcfb='xvc file bring'
alias xvcfh='xvc file hash'
alias xvcfco='xvc file checkout'
alias xvcfr='xvc file recheck'
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
```

If there are aliases that you'd rather not use with Xvc, you can [unalias](https://man7.org/linux/man-pages/man1/unalias.1p.html) them.


