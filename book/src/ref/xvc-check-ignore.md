# xvc check-ignore


## Synopsis 

```text
{{#include xvc-check-ignore.txt}}
```

## Purpose

`xvc check-ignore` is used to check whether a path is ignored by Xvc.

## Examples

By default it checks the files supplied from `stdin`.

```shell
$ xvc check-ignore
my-dir/my-file
```

If you supply paths from the CLI, they are checked instead. 

```shell
$ xvc check-ignore my-dir/my-file another-dir/another-file
```

If you're looking which `.xvcignore` file ignores (or whitelists) a certain path, you can use `--details`.

```shell
$ xvc check-ignore --details my-dir/my-file another-dir/another-file
```

`.xvcignore` file format is identical to [`.gitignore` file format](https://git-scm.com/docs/gitignore).
This utility can be used to check any other ignore rules in other files as well.
You can specify an alternative ignore filename with `--ignore-filename` option.
The below command is identical to `git check-ignore` and should give the same results.

```shell
$ xvc check-ignore --ignore-filename .gitignore 
```



