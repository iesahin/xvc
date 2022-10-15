# xvc root

Shows the Xvc root directory where `.xvc/` resides.

## Synopsis 

```text
{{#include xvc-root.txt}}
```

## Purpose

`xvc root` can be used in scripts to make paths relative to the Xvc project root.

## Examples

By default, it shows the relative path. 

```shell
$ xvc root
..
```

When you supply `--absolute`, it prints the absolute path.

```shell
$ xvc root --absolute
/home/user/my-xvc-project/
```



