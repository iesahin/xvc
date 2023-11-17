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
-rwxr-xr-x  1 iex  staff       369 Nov 18 01:55 create-subsets.zsh

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
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:56 chinese_mnist.zip -> [CWD]/.xvc/b3/b24/2c9/422f91b804ea3008bc0bc025e97bf50c1d902ae7a0f13588b84f59023d/0.zip
-rwxr-xr-x  1 iex  staff  369 Nov 18 01:55 create-subsets.zsh

```

The long directory name is the BLAKE-3 hash of the data file.

As we'll work with the file contents, let's unzip the data file.

```console
$ unzip -q chinese_mnist.zip

$ ls -l
total 8
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:56 chinese_mnist.zip -> [CWD]/.xvc/b3/b24/2c9/422f91b804ea3008bc0bc025e97bf50c1d902ae7a0f13588b84f59023d/0.zip
-rwxr-xr-x  1 iex  staff  369 Nov 18 01:55 create-subsets.zsh
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
SS         192 2023-11-17 22:56:58 3a714d65          data/data/input_9_9_9.jpg
SS         192 2023-11-17 22:56:57 9ffccc4d          data/data/input_9_9_8.jpg
SS         192 2023-11-17 22:56:58 5d6312a4          data/data/input_9_9_7.jpg
SS         192 2023-11-17 22:56:59 7a0ddb0e          data/data/input_9_9_6.jpg
SS         192 2023-11-17 22:56:57 2047d7f3          data/data/input_9_9_5.jpg
SS         192 2023-11-17 22:56:58 10fcf309          data/data/input_9_9_4.jpg
SS         192 2023-11-17 22:56:58 0bdcd918          data/data/input_9_9_3.jpg
SS         192 2023-11-17 22:56:57 aebcbc03          data/data/input_9_9_2.jpg
SS         192 2023-11-17 22:56:58 38abd173          data/data/input_9_9_15.jpg
SS         192 2023-11-17 22:56:57 7c6a9003          data/data/input_9_9_14.jpg
SS         192 2023-11-17 22:56:59 a9f04ad9          data/data/input_9_9_13.jpg
SS         192 2023-11-17 22:56:57 2d372f95          data/data/input_9_9_12.jpg
SS         192 2023-11-17 22:56:58 8fe799b4          data/data/input_9_9_11.jpg
SS         192 2023-11-17 22:56:57 ee35e5d5          data/data/input_9_9_10.jpg
SS         192 2023-11-17 22:56:58 7576894f          data/data/input_9_9_1.jpg
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


We'll use the following shell script to create subsets.

```console
$ cat create-subsets.zsh
#!/usr/bin/env zsh

for p in {1..60} ; do echo "Copying ${p}" ; xvc file copy --name-only 'data/data/input_${p}_*' data/train/ ; done

for p in {61..80} ; do echo "Copying ${p}" ; xvc file copy --name-only 'data/data/input_${p}_*' data/validate/ ; done

for p in {81..100} ; do echo "Copying ${p}" ; xvc file copy --name-only 'data/data/input_${p}_*' data/test/ ; done

```

```console
$ zsh create-subsets.zsh
? 2
Copying 1
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 2
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 3
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 4
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 5
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 6
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 7
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 8
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 9
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 10
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 11
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 12
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 13
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 14
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 15
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 16
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 17
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 18
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 19
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 20
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 21
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 22
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 23
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 24
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 25
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 26
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 27
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 28
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 29
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 30
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 31
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 32
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 33
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 34
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 35
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 36
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 37
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 38
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 39
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 40
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 41
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 42
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 43
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 44
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 45
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 46
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 47
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 48
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 49
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 50
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 51
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 52
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 53
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 54
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 55
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 56
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 57
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 58
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 59
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 60
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 61
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 62
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 63
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 64
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 65
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 66
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 67
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 68
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 69
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 70
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 71
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 72
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 73
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 74
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 75
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 76
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 77
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 78
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 79
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 80
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 81
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 82
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 83
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 84
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 85
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 86
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 87
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 88
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 89
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 90
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 91
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 92
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 93
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 94
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 95
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 96
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 97
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 98
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 99
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
Copying 100
error: unexpected argument '--name-only' found

  tip: to pass '--name-only' as a value, use '-- --name-only'

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.

```

If you look at the contents of these directories, you'll see that they are
symbolic links to the same files we started to track. 

```console
$ ls -l data/train/
total 0
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_10_1.jpg -> [CWD]/.xvc/b3/eb8/07e/554657a5c497cdfe1a4c03ab54d201e8ad49abdf9d54a5761a8a91bc30/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_10_10.jpg -> [CWD]/.xvc/b3/e97/6cd/bc0bc1e6f4402458d74ab25a5e0ce7694e257d61f318c1c079c9ef6dbb/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_10_11.jpg -> [CWD]/.xvc/b3/4b9/9c0/8c1bb2abed0a8369cb8ea7b587782eb00ff15a68294f4280517a1d3909/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_10_12.jpg -> [CWD]/.xvc/b3/886/e9a/092d6a88e48f3cc8dbc4440b933fbaba2802d36e56769d867b7da9f749/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_10_13.jpg -> [CWD]/.xvc/b3/0d9/37b/6f4a55e6e53656cd8ef574f93fbb11927af228ec0749c1f5b85d566d9e/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_10_14.jpg -> [CWD]/.xvc/b3/667/d34/0e1448da8a030beca3fa668582cf7c65d1caaafe19047dc201350e1189/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_10_15.jpg -> [CWD]/.xvc/b3/5d2/029/e65c2e6d1df0423d837ba9bb64d0ce9d63cea9473285530c73789b00f2/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_10_2.jpg -> [CWD]/.xvc/b3/e3a/11b/c94daff7ae8d08c63361273e34c06426903a204a84aa0522a9aad14824/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_10_3.jpg -> [CWD]/.xvc/b3/5b6/139/f82fa0a32ab567962aacaf808a9fdb79cf7bf51208bd8acb072904da67/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_10_4.jpg -> [CWD]/.xvc/b3/2fa/cc6/3147781a008fc6b1d24a11aecd5d1e50aeaad7e97eca9924262a1aecf2/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_10_5.jpg -> [CWD]/.xvc/b3/6b3/4b1/5ec9df5a925beb78eff2388a566a6ca897e5dac32c16ae69046e7505d2/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_10_6.jpg -> [CWD]/.xvc/b3/2de/7cf/78eb30f66c17df1f0c0ee86ddb59b9af1f7c06747b503df18829ce8fe1/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_10_7.jpg -> [CWD]/.xvc/b3/5ef/643/f49f8b602ba105a355bcacbdd420b87b24fde03bac5501c3f2f1beca8d/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_10_8.jpg -> [CWD]/.xvc/b3/eb6/d51/d453d28b3c8999c8b8db2d7a2f0398985ad8f01a137785c3f50ebb5856/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_10_9.jpg -> [CWD]/.xvc/b3/e10/ff3/92509d7f6604da629f3c06e460508cbab024079ae3c6309a5d5a8161a5/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_1_1.jpg -> [CWD]/.xvc/b3/8af/979/820efcf27bec542173cfc80d583ecf528a16d2d821f6d80d98aca22529/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_1_10.jpg -> [CWD]/.xvc/b3/139/8d2/46227ca6704c2c695514ab1b7afb90ad14a1280022373ae1ff0e7ddd58/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_1_11.jpg -> [CWD]/.xvc/b3/401/ce8/42f4c7d19f8d4efae3efb912e9362fe8df74b4eefa6ac70b1af4b96adc/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_1_12.jpg -> [CWD]/.xvc/b3/191/123/e6fa958fb3ef41c0b40344ebc98e2d74c0053c513c3b64fcec19df72be/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_1_13.jpg -> [CWD]/.xvc/b3/1b8/837/925444ae1543c308abf7514b9a20564f99f5196f1609c87839e35bd15d/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_1_14.jpg -> [CWD]/.xvc/b3/41a/a88/0a51858c5fdb92e677732a45d2468156434d1d2772f732f8325665b268/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_1_15.jpg -> [CWD]/.xvc/b3/13b/323/be4d8fc5de295295b3dbcd05210c2f0ae4394a95e234f6673833c862f8/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_1_2.jpg -> [CWD]/.xvc/b3/eaf/7ca/0e7fa6923d9b3007c108f39cc5f2773c67d4f718937c05f69559807aee/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_1_3.jpg -> [CWD]/.xvc/b3/3c0/111/f3321c1a5240958df26b9a9806d48fb37d7d3b59968c32cf1cddd584eb/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_1_4.jpg -> [CWD]/.xvc/b3/daf/6bb/7220480567cca62068283cb170b2acc80df77566285692a04b7f4db71c/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_1_5.jpg -> [CWD]/.xvc/b3/083/2e6/ec7f5734103ae93dc6ea1410856bbf1d16f52c7fe82c2eeb43fe5ee809/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_1_6.jpg -> [CWD]/.xvc/b3/46c/6f1/817456df31d8f6667455238569c5bde6c6ecdd58c76fcc85de8ca4162f/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_1_7.jpg -> [CWD]/.xvc/b3/5b6/886/59111a5044f153ffd6aa2df124ce119a1354c7bbfb09556c871f352fe3/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_1_8.jpg -> [CWD]/.xvc/b3/511/be2/d173e6b66bf682ef5347cc489cf2558288ce734d4bf63a6713b4d3c243/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_1_9.jpg -> [CWD]/.xvc/b3/2f1/211/df479bbe2756182133db2a316d43e05ff4888fc0ce86556f14c0d3a8da/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_2_1.jpg -> [CWD]/.xvc/b3/a08/c92/e958071e096ac6c92ba6966147477f062da8898c49da8a2fea7e3d3b13/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_2_10.jpg -> [CWD]/.xvc/b3/6cf/3bd/3fc02b6691fe952c29d5943b656db187fc68b8e54a11ac8f9884c25953/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_2_11.jpg -> [CWD]/.xvc/b3/bb3/3c3/d2ce3bae4a3767ff129db009a623489d8b09155d04bbb46b90873361d8/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_2_12.jpg -> [CWD]/.xvc/b3/e68/0e1/0ee89b8becb1b67f06ffa6506d63713ecc707d2f8a94fe37c3ed3ffb8b/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_2_13.jpg -> [CWD]/.xvc/b3/0a6/ff8/32a649224b42cd8321af5ec40ae2343e916b09c5ec51d60b6c25a70af0/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_2_14.jpg -> [CWD]/.xvc/b3/8af/dd7/791da397e5c3039d8294c69e95f70431d4257b197bde3f317c5768e475/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_2_15.jpg -> [CWD]/.xvc/b3/c45/e7c/a0b9e7ff554ba26e0720c0facd368c22c92dec363fdf0ebcd909d7b47d/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_2_2.jpg -> [CWD]/.xvc/b3/b52/c57/2fb304dfd27512f718d0c235b15ff4e098b188924b527c068cae0475cc/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_2_3.jpg -> [CWD]/.xvc/b3/2e9/70b/0428fae6afac73eb9ff04ee1de24364614cc8939878b19c1e0429b95ca/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_2_4.jpg -> [CWD]/.xvc/b3/877/22a/da1fee8359a31c9b080bb433a4a00169fcd389e739961f0c642c0a75db/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_2_5.jpg -> [CWD]/.xvc/b3/a75/3ff/788cdf6a33b2332118ae27876168cc7ff5a1d70e8a3b856920205ce811/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_2_6.jpg -> [CWD]/.xvc/b3/2a8/7d1/51f7952479ede3bae56c17b8217fc9df5ea21fd142ff9c2edb6767e16f/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_2_7.jpg -> [CWD]/.xvc/b3/06b/384/b30ebc09fd42d1a491fd6a5092fb646b69540c5afd2bfba2762eb368de/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_2_8.jpg -> [CWD]/.xvc/b3/46c/37a/ce8bef9014cecc088ea08b8ea723c34e9ff8ea0a0f41a0425ee89b15a5/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_2_9.jpg -> [CWD]/.xvc/b3/030/f50/cb8cdc8a5098ca65d56bc5e78293201429d4238db8e5f85256abb4fa7d/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_3_1.jpg -> [CWD]/.xvc/b3/8d9/8a0/6c8e6de9d073ea75f3726a1bd2491ce4cc7eb4124f3119036cc0801ea9/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_3_10.jpg -> [CWD]/.xvc/b3/8c7/551/e1c7d57cc9246dbef66c6daf83b98cf977c12474bda6baceef4c789412/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_3_11.jpg -> [CWD]/.xvc/b3/a2d/463/64f50f31f0a3377171df6f7fb77897033f6c4287769557a33eed1c0230/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_3_12.jpg -> [CWD]/.xvc/b3/1f2/7f8/b234c815d875eef0f8d1b81e660049819955da2d62bac0e340305dda48/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_3_13.jpg -> [CWD]/.xvc/b3/498/387/d335d8d8cb9ce62536fc522bbafda4c332d04b9c0aab6dd061a5581ea7/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_3_14.jpg -> [CWD]/.xvc/b3/4bb/e0e/f892d0964946b87165e29107cad6a412f60d98ed882e6d6cb313598905/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_3_15.jpg -> [CWD]/.xvc/b3/5df/27b/c4cc4f0c4affcae2583abf4bc0e5db289e86b1e3a29d0fbe35ae766024/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_3_2.jpg -> [CWD]/.xvc/b3/9b2/d42/d2196b3191ba5aec09d59c6b473aaadea59cecd884dcf6e6e91f070620/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_3_3.jpg -> [CWD]/.xvc/b3/903/91f/a5423e76dc9bf275e75a9a6860504c0c1d7dc6f5f683ca051a0f0e45d2/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_3_4.jpg -> [CWD]/.xvc/b3/3b3/44f/c1dd6b3e2433fdc7e38174b8606bcf16c1279d0bb763e21f76ceaaff7c/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_3_5.jpg -> [CWD]/.xvc/b3/2e6/daf/5d5cf009112c21bf3bcd0769d176677caaba0f9d8a189fcff6e83aba22/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_3_6.jpg -> [CWD]/.xvc/b3/621/881/26b8291d6c314109878f10cc6ad12076c44f9d15ab9cdaa76403234b0d/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_3_7.jpg -> [CWD]/.xvc/b3/270/6e4/86a5c94a46ef6cb5bb0be96ed2ed5ef37d203be92401fb9f724f7d6f1b/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_3_8.jpg -> [CWD]/.xvc/b3/e3b/41d/90f400fc15f1bd6b668008e93911f223e0866e0dfad1cd614868e8b6ac/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_3_9.jpg -> [CWD]/.xvc/b3/f8e/76c/0d3971e9a2a137e2dc518b5602018e11320ff0985195e7f84163e63219/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_4_1.jpg -> [CWD]/.xvc/b3/bc2/da7/71b4efe5e8ba3a4d9b0f558abbd802278613e4aae75ff0e5f24aa683f2/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_4_10.jpg -> [CWD]/.xvc/b3/7b7/f74/6c6cb7cf1b702356be474030bfa104e2d92a659c4a1b8791acb25ef583/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_4_11.jpg -> [CWD]/.xvc/b3/fc9/0c7/b9631b516f84aa7f0209ba4e785a305f9b91eaa8145be2636f703b3790/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_4_12.jpg -> [CWD]/.xvc/b3/758/03a/e2f8f96b0ae05cc1daa8b83513935f8ea4366a25fe44ff21dc8e188d24/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_4_13.jpg -> [CWD]/.xvc/b3/054/b4e/ddbd48ac5bc509151a471a5bd887056497e549623910ae0f3eaa3ed046/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_4_14.jpg -> [CWD]/.xvc/b3/3d3/772/c9b2129008eb77018fe2c3bda4c17768b1ca99af3fb6ee4c4c873c6c2c/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_4_15.jpg -> [CWD]/.xvc/b3/d4c/6ba/bddf232b925652248d86cb2722787d63925e7808a5acfe700402aed114/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_4_2.jpg -> [CWD]/.xvc/b3/221/220/af1a354a45274278338e22cd1f772690a05f83cd9b4b70c0fd9f669beb/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_4_3.jpg -> [CWD]/.xvc/b3/ffb/655/4876f486135a38f170c36e38647159a5852829716123d1fa661e3f97d7/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_4_4.jpg -> [CWD]/.xvc/b3/ced/cce/696a2e639e84f8c967d7a1cd6fe58539bdc0edebdfd7f94b9994a348e3/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_4_5.jpg -> [CWD]/.xvc/b3/c66/c1e/155f90610e67973d91e2f591f8e153f2114efe06662522a27558770cec/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_4_6.jpg -> [CWD]/.xvc/b3/be2/2aa/7c9a17441b09f49c26c1448f571deea22bec73e9bdb5ae8bcb9c2f279d/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_4_7.jpg -> [CWD]/.xvc/b3/c03/0a5/fde3abb7a788eb71e6342a53d7b852b5133f1b32823eff1a45f19769fa/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_4_8.jpg -> [CWD]/.xvc/b3/239/537/b16f9c2dca1474f1edfc7ed7b2fa4ed58bd79e0e328d64ca30789855e2/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_4_9.jpg -> [CWD]/.xvc/b3/43e/32f/8b7b2c7a3ad1dc93a10b987e9cdcb3f26e89726a8d5ccafe895438d970/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_5_1.jpg -> [CWD]/.xvc/b3/32c/8a9/4e916415f22ab2ba9fd3fe82e458d0f9d53bb18e29af10d7b77c5e6cbb/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_5_10.jpg -> [CWD]/.xvc/b3/2c9/12d/440879e347b4b6120fb7e77624d660d3a77c21bc56297fc338b85098bb/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_5_11.jpg -> [CWD]/.xvc/b3/48b/3f4/764d7aaf59eff91a700f8c2ce44fa4b2faaa7f657dcc148a8422167a53/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_5_12.jpg -> [CWD]/.xvc/b3/296/f10/0aaca172d7b0c5a08ad0c4bce786f8174888a92ad3f4a23ad268aaed4c/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_5_13.jpg -> [CWD]/.xvc/b3/d7a/439/1231415ed4252f3aa1463d881958f3616d28b85770054420c62680cc2b/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_5_14.jpg -> [CWD]/.xvc/b3/fe1/f09/d40fd4750c995249da69f691327a9be727eb92ee38b9cbed36506eed14/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_5_15.jpg -> [CWD]/.xvc/b3/13c/84e/1bd7961d4d555c2aa95072242c63431f34348a3a1ff65e0f44f2ec3690/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_5_2.jpg -> [CWD]/.xvc/b3/d8e/3c1/4006e2badb5dc823db43c12678f08e74484d5de2d21be2626dae2851e0/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_5_3.jpg -> [CWD]/.xvc/b3/3ed/09d/07ec8ec9ce90a43b14ee02cdeca5eefb6192f9d0edea684be8c4d37bd1/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_5_4.jpg -> [CWD]/.xvc/b3/b95/2e4/6832746cb09be2f74bcb8c3a3dd92b8fc69af7ad098ce1927bfede91c5/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_5_5.jpg -> [CWD]/.xvc/b3/600/c19/99928c7320d4769ad78d25bf163625749ad1c1de7e7164567b2a78e78e/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_5_6.jpg -> [CWD]/.xvc/b3/99e/995/88c535ef3f4f3c11ad2a09fad0f609bb154e668ab2d0cbb3dad10e110e/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_5_7.jpg -> [CWD]/.xvc/b3/0f7/a84/b2b2b4a7fe9e68316a9ba8c34b5454ae302d9921a4bb25af0ebcb3311d/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_5_8.jpg -> [CWD]/.xvc/b3/e69/ad8/121ac944c6a8a49ebc12f1d487c858079533cc4d6faff9d7fc2a38c7a7/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_5_9.jpg -> [CWD]/.xvc/b3/525/737/2b8fb7d4408b33e6dd8d4740bba9c0cfcc99ab56653ef518c6eba69a26/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_6_1.jpg -> [CWD]/.xvc/b3/dc3/b71/b5f0790e671fc02ec0c4262a4cc2558e73398f407e9fbfec70aa5c36ab/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_6_10.jpg -> [CWD]/.xvc/b3/9e5/494/86f546a00c5b538053c33ed96f5c7cf55b2afa4ead5a6f3d96805de0dd/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_6_11.jpg -> [CWD]/.xvc/b3/2a6/9c3/363809985c63ac61f2423385f3662e7564074050164ce722ff0e277390/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_6_12.jpg -> [CWD]/.xvc/b3/39f/747/b9bb29a572497f36e0f898f2be6e91792e440969224f75b1c47818a5fe/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_6_13.jpg -> [CWD]/.xvc/b3/1d4/479/266bcf66455c426f3aac710bf74acfc20ff61c2c6f781f7078fcca5acc/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_6_14.jpg -> [CWD]/.xvc/b3/443/517/d0b01180b6386b5599662d730ebdd40a3d14d0459a5ec99c45d6022c6f/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_6_15.jpg -> [CWD]/.xvc/b3/f8f/906/5fbcb69d686947ace3c6f6f1be7da2f0b9a9b8be340b9a07027f0d55b5/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_6_2.jpg -> [CWD]/.xvc/b3/1ce/5b5/908b0b2b6459dabe3ee3e2011db9241074d483739eeaa28dce745c4990/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_6_3.jpg -> [CWD]/.xvc/b3/2bc/867/da86f0d0d7fb12bc4b7971209d20f7efcec7fc7c6c59ff3e3be20ce52b/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_6_4.jpg -> [CWD]/.xvc/b3/e6a/988/fa3b94aa489386ce1c21d27470acf14788bd6ba7fc4343f0543524db19/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_6_5.jpg -> [CWD]/.xvc/b3/a63/31e/f7454a37c887c68b600f099d0519ddf12588ed18d864b759428816308f/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_6_6.jpg -> [CWD]/.xvc/b3/ca0/950/500519bb823edd372f7c7363036c320200af04c4c9a397cbf3416cdd31/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_6_7.jpg -> [CWD]/.xvc/b3/f16/b9c/7fea5c6a2a93243387d8ee0b935ad57d3350e08daa97d51b8a2ebf515f/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_6_8.jpg -> [CWD]/.xvc/b3/8a9/787/ff35f2e38e65fe0db73e02a915209eafe61012594284018cb4b1a18102/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_6_9.jpg -> [CWD]/.xvc/b3/206/625/aee087b4233d34dbc8d52734213714a42aa44909916063c9f0d56c0376/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_7_1.jpg -> [CWD]/.xvc/b3/496/6ea/f6dac1bc61c84b4e75b955e3afbb8791c576138fa42c2e82e7e2f7a5c1/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_7_10.jpg -> [CWD]/.xvc/b3/410/0c3/a947f03ade27ff41db58113730f138829f7b8cc7450498b972c0fd8028/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_7_11.jpg -> [CWD]/.xvc/b3/808/c1f/dee4e645c050738b8c0f6b6f49eb0755495981125b952222480d5c531f/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_7_12.jpg -> [CWD]/.xvc/b3/a0d/930/cb48d82785114ee75260633af1677a9b11563beaa0a5ea9748a730be87/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_7_13.jpg -> [CWD]/.xvc/b3/1e0/12d/8fc3c2163c42c042e1f5181003903cc47e3a68afe444507d7780861053/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_7_14.jpg -> [CWD]/.xvc/b3/879/c0f/086a742a3eff16f4acea1cfec4164ec3a88fd47ec9b79705f814c6cd22/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_7_15.jpg -> [CWD]/.xvc/b3/f1a/ab3/b918839a12449096e1d55a944058ab45655c4ca4abf63fee9554d8eaaa/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_7_2.jpg -> [CWD]/.xvc/b3/d85/3ea/0cbe4d2656e6dec7a9e17ec204a0f233c0c29f820451171f0f9ef6b043/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_7_3.jpg -> [CWD]/.xvc/b3/6c6/274/e180b76554ceea95b041c369bc852afed7ee70594ca0f6d60c54e5d68c/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_7_4.jpg -> [CWD]/.xvc/b3/120/fba/02c3946a28f3ef9f7bcf8edf3a28c2ff082edd644eb0d814f78a5a71b2/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_7_5.jpg -> [CWD]/.xvc/b3/759/773/11ebae8415c20e54cf5e2b281385bd6e0ecccd58e3d5c1a2502e14978f/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_7_6.jpg -> [CWD]/.xvc/b3/097/bdb/5cec3ed442d9e798339028360bca97ce67ca2e6a664bf9e20635537552/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_7_7.jpg -> [CWD]/.xvc/b3/b54/637/ee292fc213cad3de1de06700eeab4635f0ac6da86f99eb4b665a58ce61/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_7_8.jpg -> [CWD]/.xvc/b3/de3/cb9/97c70fb811ce66583c3ceb12ca91a4d66399ca644b37b83eed6669ca58/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_7_9.jpg -> [CWD]/.xvc/b3/533/188/d6dd8a8f3f6fafbc0c81c50053572c5c66a524401d29a345109e5c0fb2/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_8_1.jpg -> [CWD]/.xvc/b3/1de/456/751c3a1a7e05b80c97cabc01891de71441318f1e29b73e7443503646c8/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_8_10.jpg -> [CWD]/.xvc/b3/26e/4d1/64e4b7cb6dc2d8f86b1572fff5c622bf195248f35556d2e7ce34dcc8e1/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_8_11.jpg -> [CWD]/.xvc/b3/257/e32/132cff4da44060ea31e65ad9e30ddb394c5e6c190dd923fe54f5692aa9/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_8_12.jpg -> [CWD]/.xvc/b3/de0/b8e/4d4815c031450ee8cb45636742401da0786d5e782f90974ecbcb6e222f/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_8_13.jpg -> [CWD]/.xvc/b3/16e/8e4/4f944bae5a59d68c608a761672854decc110c8e3a9e6d1ae49beb4d995/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_8_14.jpg -> [CWD]/.xvc/b3/535/c62/fdff432845999220e996b360af1c53c1c2bbf624b6230f824abf259f11/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_8_15.jpg -> [CWD]/.xvc/b3/841/bbc/c1d0c302373a1c9510700b636222525a76c31301b90b78e7b4e6106945/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_8_2.jpg -> [CWD]/.xvc/b3/a8e/b0b/2aab8635d01b57905a4accb6e462eee1eda4b3ddf4851927067c8d94bf/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_8_3.jpg -> [CWD]/.xvc/b3/0a0/40a/7eb6d8d07b379b4048f44011c807358c1a901b8e14922c3be5e01a6582/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_8_4.jpg -> [CWD]/.xvc/b3/783/eeb/e2ead3b15e64a179b967ecd63f6ad62b677838bb3c419f93bdb094994c/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_8_5.jpg -> [CWD]/.xvc/b3/73a/5d9/cacad7af1daf4c2b3b661bdb5cd99df3448af3a84e96caa2e663b860c2/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_8_6.jpg -> [CWD]/.xvc/b3/79a/4b1/c5596a089d5aec32a40c878b09d35e87975023e9879dc3f48eecac400f/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_8_7.jpg -> [CWD]/.xvc/b3/e59/dbe/5894b23c39c16daa9a62526ed037cc3539f9a0572df52c2c832baf1b44/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_8_8.jpg -> [CWD]/.xvc/b3/06e/0cd/3ad6eb1460efa63647ff0edf7b0c2ca65f12307d68909a4b837e7cfe12/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_8_9.jpg -> [CWD]/.xvc/b3/34c/477/45f49465e4650706d8f5d5333d63c646102f2ec9078055bfbb387cbf10/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_9_1.jpg -> [CWD]/.xvc/b3/4df/6c8/ed10f4513aebcb48feb38a084615bab55b528875e62ffac0535ed13881/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_9_10.jpg -> [CWD]/.xvc/b3/02f/fcd/70333ad43398fa660f1bf5370a240661abd97b29b09fa72fda8e40a798/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_9_11.jpg -> [CWD]/.xvc/b3/f9a/629/2a17f90565af849e47eebc925e629dda0aaf200da6e5355e00c6bbe96f/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_9_12.jpg -> [CWD]/.xvc/b3/0d5/6c6/42b1f0638dfef17dbca83f8c44885b7e7f49c5c52b57c1cf84463a9f9c/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_9_13.jpg -> [CWD]/.xvc/b3/366/382/a34c4de72f328f1db9a0c7498839ac3d2e5ad45a7916379a5a774007bf/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_9_14.jpg -> [CWD]/.xvc/b3/698/00d/a2c36b79344351fae80314b9f65ed7b9dc476ab902fb8c93916cf9c38b/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_9_15.jpg -> [CWD]/.xvc/b3/af8/4d0/d171e28b12690b327a4f0dcff5cb48fd1586e84c70299f40ee3f083029/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_9_2.jpg -> [CWD]/.xvc/b3/78b/882/684ad540fb1538ea3cbcf22018da217061012309d478a0ce0480ad56dc/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_9_3.jpg -> [CWD]/.xvc/b3/bac/682/ba8125bc0a0b6fc80f22b9854e189a2f087513332ec5bbc0b9166b7fb3/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_9_4.jpg -> [CWD]/.xvc/b3/a6c/684/9bac1b2f015aaad4e0e4a9b2e5f2cc58a15ade82966dcd65a0aecadf2f/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_9_5.jpg -> [CWD]/.xvc/b3/e8c/2e9/6215ef7741d02375ff17d3023fdf6a22c56c3736fa5c69565388684f34/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_9_6.jpg -> [CWD]/.xvc/b3/ad3/3ca/20f0a70852a54eb3ee6659e5adf15ebde819aa44e2c66184879b26a8af/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_9_7.jpg -> [CWD]/.xvc/b3/1de/349/779c9d35bdca76785d757b4f1cea6ad160f34ac343055122f12d758d8b/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_9_8.jpg -> [CWD]/.xvc/b3/6d4/a7a/b6f651f389f93094ad95af472549d6b3fea99e6d8fc06dd80e621d35e7/0.jpg
lrwxr-xr-x  1 iex  staff  192 Nov 18 01:57 input_1_9_9.jpg -> [CWD]/.xvc/b3/54a/027/4139379af1d40fcbed4c3d691783d434923e00d07543f481e7419b947f/0.jpg

```
