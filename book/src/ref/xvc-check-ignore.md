# xvc check-ignore

## Purpose

Check whether a path is ignored or whitelisted by Xvc.

## Synopsis

```console
$ xvc check-ignore --help
Check whether files are ignored with `.xvcignore`

Usage: xvc check-ignore [OPTIONS] [TARGETS]...

Arguments:
  [TARGETS]...
          Targets to check. If no targets are provided, they are read from stdin

Options:
      --ignore-filename <IGNORE_FILENAME>
          Filename that contains ignore rules

          This can be set to .gitignore to test whether Git and Xvc work the same way.

          [default: .xvcignore]

  -h, --help
          Print help (see a summary with '-h')

```

## Examples

```console
$ git init
...
$ xvc init
```

You can add files and directories to be ignored by Xvc to `.xvcignore` files.

```console
$ zsh -cl "echo 'my-dir/my-file' >> .xvcignore"
```

By default it checks the files supplied from `stdin`.

```console
$ zsh -cl 'echo my-dir/my-file | xvc check-ignore'
[IGNORE] [CWD]/my-dir/my-file

```

The `.xvcignore` file format is identical to [`.gitignore` file format](https://git-scm.com/docs/gitignore).

```console
$ cat .xvcignore

# Add patterns of files xvc should ignore, which could improve
# the performance.
# It's in the same format as .gitignore files.

.DS_Store
my-dir/my-file

```

If you supply paths from the CLI, they are checked against the ignore rules in `.xvcignore`.

```console
$ xvc check-ignore my-dir/my-file another-dir/another-file
[IGNORE] [CWD]/my-dir/my-file
[NO MATCH] [CWD]/another-dir/another-file

```

You can also add whitelist patterns to `,.xvcignore` files.

```console
$ zsh -cl "echo '!another-dir/*' >> .xvcignore"
```

```console
$ xvc check-ignore my-dir/my-file another-dir/another-file
```

This utility can be used to check any other ignore rules in other files as well.
You can specify an alternative ignore filename with `--ignore-filename` option.
The below command is identical to `git check-ignore` and should give the same results.

```console
$ xvc check-ignore --ignore-filename .gitignore

```
