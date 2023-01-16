# xvc root

## Purpose

Shows the Xvc root project directory where `.xvc/` resides.

## Synopsis 

```console
$ xvc root --help
Find the root directory of a project

Usage: xvc root [OPTIONS]

Options:
      --absolute  Show absolute path instead of relative
  -h, --help      Print help

```


## Examples

`xvc root` can be used in scripts to make paths relative to the Xvc project root.

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



