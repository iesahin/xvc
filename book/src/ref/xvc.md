# Command Reference

## Synopsis

```console
$ xvc --help
Xvc CLI to manage data and ML pipelines

Usage: xvc [OPTIONS] <COMMAND>

Commands:
  file          File and directory management commands
  init          Initialize an Xvc project
  pipeline      Pipeline management commands
  storage       Storage (cloud) management commands
  root          Find the root directory of a project
  check-ignore  Check whether files are ignored with `.xvcignore`
  aliases       Print command aliases to be sourced in shell files
  help          Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...             Output verbosity. Use multiple times to increase the output detail
      --quiet                  Suppress all output
      --debug                  Turn on all logging to $TMPDIR/xvc.log
  -C <WORKDIR>                 Set working directory for the command. It doesn't create a new shell, or change the directory [default: .]
  -c, --config <CONFIG>        Configuration options set from the command line in the form section.key=value You can use multiple times
      --no-system-config       Ignore system configuration file
      --no-user-config         Ignore user configuration file
      --no-project-config      Ignore project configuration file (.xvc/config)
      --no-local-config        Ignore local (gitignored) configuration file (.xvc/config.local)
      --no-env-config          Ignore configuration options obtained from environment variables
      --skip-git               Don't run automated Git operations for this command. If you want to run git commands yourself all the time, you can set `git.auto_commit` and `git.auto_stage` options in the configuration to False
      --from-ref <FROM_REF>    Checkout the given Git reference (branch, tag, commit etc.) before performing the Xvc operation. This runs `git checkout <given-value>` before running the command
      --to-branch <TO_BRANCH>  If given, create (or checkout) the given branch before committing results of the operation. This runs `git checkout --branch <given-value>` before committing the changes
  -h, --help                   Print help information
  -V, --version                Print version information

```

## Subcommands

- [`file`](./xvc-file.md): File and directory management commands
- [`init`](./xvc-init.md): Initialize an Xvc project
- [`pipeline`](./xvc-pipeline.md): Pipeline management commands
- [`storage`](./xvc-storage.md): Storage (cloud) management commands
- [`root`](./xvc-root.md): Find the root directory of a project
- [`check-ignore`](./xvc-check-ignore.md): Check whether files are ignored with `.xvcignore`
- [`aliases`](./xvc-aliases.md) Print command aliases to be sourced in shell files
 
