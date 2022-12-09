# xvc file carry-in

Copies the file changes to cache. 

## Synopsis

```console
$ xvc file carry-in --help

```


## Examples


```console
$ git init
...
$ xvc init

$ xvc file track data.txt

$ xvc file list

$ sed -i 's/a/e' data.txt

$ xvc file carry-in data.txt

```