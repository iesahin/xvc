# How to create a data pipeline with Xvc

A data pipeline starts from data and ends with models. 

```console
$ git init
Initialized empty Git repository in [CWD]/.git/

$ xvc init
```

In this HOWTO, we use Chinese MNIST dataset to create an image classification pipeline. We already downloaded it [from kaggle](https://www.kaggle.com/datasets/gpreda/chinese-mnist/data). 

```console
$ ls -l
total 21088
-rw-r--r--  1 iex  staff  10792680 Nov 17 19:46 chinese_mnist.zip
-rwxr-xr-x  1 iex  staff       267 Nov 18 00:16 create-subsets.zsh

```
Let's start by tracking the data file with Xvc.

```console
$ xvc file track chinese_mnist.zip --as symlink

```

The default [recheck (checkout) method](/ref/xvc-file-recheck.md) is _copy_ that means the file is
duplicated in the workspace as a writable file. We don't need to write over this
data file, we'll only read from it, so we set the recheck type as symlink.

```console
$ ls -l
total 8
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:49 chinese_mnist.zip -> [CWD]/.xvc/b3/b24/2c9/422f91b804ea3008bc0bc025e97bf50c1d902ae7a0f13588b84f59023d/0.zip
-rwxr-xr-x  1 iex  staff  267 Nov 18 00:16 create-subsets.zsh

```

The long directory name is the BLAKE-3 hash of the data file.

As we'll work with the file contents, let's unzip the data file.

```console
$ unzip -q chinese_mnist.zip

$ ls -l
total 8
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:49 chinese_mnist.zip -> [CWD]/.xvc/b3/b24/2c9/422f91b804ea3008bc0bc025e97bf50c1d902ae7a0f13588b84f59023d/0.zip
-rwxr-xr-x  1 iex  staff  267 Nov 18 00:16 create-subsets.zsh
drwxr-xr-x  4 iex  staff  128 Nov 17 19:45 data

```

Now we have the data directory with the following structure:

```console
$ tree -d data
data
└── data

2 directories

```

Let's track the data directory as well with Xvc.

```console
$ xvc file track data --as symlink
```

The reason we're tracking the data directory separately is that we'll use different subsets as training, validation, and test data. 

Let's list the track status of files first. 

```console
$ xvc file list data/data/input_9_9_*
SS         192 2023-11-17 22:49:28 3a714d65          data/data/input_9_9_9.jpg
SS         192 2023-11-17 22:49:27 9ffccc4d          data/data/input_9_9_8.jpg
SS         192 2023-11-17 22:49:28 5d6312a4          data/data/input_9_9_7.jpg
SS         192 2023-11-17 22:49:28 7a0ddb0e          data/data/input_9_9_6.jpg
SS         192 2023-11-17 22:49:26 2047d7f3          data/data/input_9_9_5.jpg
SS         192 2023-11-17 22:49:26 10fcf309          data/data/input_9_9_4.jpg
SS         192 2023-11-17 22:49:28 0bdcd918          data/data/input_9_9_3.jpg
SS         192 2023-11-17 22:49:28 aebcbc03          data/data/input_9_9_2.jpg
SS         192 2023-11-17 22:49:27 38abd173          data/data/input_9_9_15.jpg
SS         192 2023-11-17 22:49:28 7c6a9003          data/data/input_9_9_14.jpg
SS         192 2023-11-17 22:49:28 a9f04ad9          data/data/input_9_9_13.jpg
SS         192 2023-11-17 22:49:27 2d372f95          data/data/input_9_9_12.jpg
SS         192 2023-11-17 22:49:28 8fe799b4          data/data/input_9_9_11.jpg
SS         192 2023-11-17 22:49:26 ee35e5d5          data/data/input_9_9_10.jpg
SS         192 2023-11-17 22:49:26 7576894f          data/data/input_9_9_1.jpg
Total #: 15 Workspace Size:        2880 Cached Size:        8710


```

`xvc file list` command shows the tracking status. Initial two characters shows
the tracking status, `SS` means the file is tracked as symlink and is available
in the workspace as a symlink. The next column shows the file size, then the
last modified date, then the BLAKE-3 hash of the file, and finally the file
name. The empty column contains the actual hash of the file if the file is
available in the workspace. Here it's empty because the workspace file is a
link. 

The summary line shows the total size of the files and the size they occupy in
the workspace.

Now, we'll create a subset of these files with `xvc file copy` comand. 

The data set contains 15 classes. It has 10 samples for each of these classes
from 100 different people. As we'll train a Chinese digit recognizer, we'll
first divide volunteers 1-60 for training, 61-80 for validation, and 81-100 for
testing. This will ensure that the model is not trained with the same person's
handwriting.

```console
$ xvc file copy --name-only data/data/input_1_* data/train/ 
? 2
error: unrecognized subcommand 'copy'

Usage: xvc [OPTIONS] <COMMAND>

For more information, try '--help'.

```

```console
$ tree data/train/
? 2
data/train/  [error opening dir]

0 directories, 0 files

```

We'll use the following shell script to create subsets.

```console
$ cat create-subsets.zsh
#!/usr/bin/env zsh

for p in {1..60} ; do xvc file copy 'data/data/input_${p}_*' data/train/ ; done

for p in {61..80} ; do xvc file copy 'data/data/input_${p}_*' data/validate/ ; done

for p in {81..100} ; do xvc file copy 'data/data/input_${p}_*' data/test/ ; done

```

```console
$ zsh create-subsets.zsh

```

If you look at the contents of these directories, you'll see that they are
symbolic links to the same files we started to track. 

```console
$ ls -l data/train/
? 1
ls: data/train/: No such file or directory

```
