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
lrwxr-xr-x  1 iex  staff  191 Nov 18 02:19 chinese_mnist.zip -> [CWD]/.xvc/b3/b24/2c9/422f91b804ea3008bc0bc025e97bf50c1d902ae7a0f13588b84f59023d/0.zip
-rwxr-xr-x  1 iex  staff  369 Nov 18 01:55 create-subsets.zsh

```

The long directory name is the BLAKE-3 hash of the data file.

As we'll work with the file contents, let's unzip the data file.

```console
$ unzip -q chinese_mnist.zip

$ ls -l
total 8
lrwxr-xr-x  1 iex  staff  191 Nov 18 02:19 chinese_mnist.zip -> [CWD]/.xvc/b3/b24/2c9/422f91b804ea3008bc0bc025e97bf50c1d902ae7a0f13588b84f59023d/0.zip
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
SS         191 2023-11-17 23:19:51 3a714d65          data/data/input_9_9_9.jpg
SS         191 2023-11-17 23:19:52 9ffccc4d          data/data/input_9_9_8.jpg
SS         191 2023-11-17 23:19:52 5d6312a4          data/data/input_9_9_7.jpg
SS         191 2023-11-17 23:19:51 7a0ddb0e          data/data/input_9_9_6.jpg
SS         191 2023-11-17 23:19:51 2047d7f3          data/data/input_9_9_5.jpg
SS         191 2023-11-17 23:19:53 10fcf309          data/data/input_9_9_4.jpg
SS         191 2023-11-17 23:19:51 0bdcd918          data/data/input_9_9_3.jpg
SS         191 2023-11-17 23:19:52 aebcbc03          data/data/input_9_9_2.jpg
SS         191 2023-11-17 23:19:51 38abd173          data/data/input_9_9_15.jpg
SS         191 2023-11-17 23:19:52 7c6a9003          data/data/input_9_9_14.jpg
SS         191 2023-11-17 23:19:52 a9f04ad9          data/data/input_9_9_13.jpg
SS         191 2023-11-17 23:19:51 2d372f95          data/data/input_9_9_12.jpg
SS         191 2023-11-17 23:19:52 8fe799b4          data/data/input_9_9_11.jpg
SS         191 2023-11-17 23:19:53 ee35e5d5          data/data/input_9_9_10.jpg
SS         191 2023-11-17 23:19:53 7576894f          data/data/input_9_9_1.jpg
Total #: 15 Workspace Size:        2865 Cached Size:        8710


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
Copying 1
Copying 2
Copying 3
Copying 4
Copying 5
Copying 6
Copying 7
Copying 8
Copying 9
Copying 10
Copying 11
Copying 12
Copying 13
Copying 14
Copying 15
Copying 16
Copying 17
Copying 18
Copying 19
Copying 20
Copying 21
Copying 22
Copying 23
Copying 24
Copying 25
Copying 26
Copying 27
Copying 28
Copying 29
Copying 30
Copying 31
Copying 32
Copying 33
Copying 34
Copying 35
Copying 36
Copying 37
Copying 38
Copying 39
Copying 40
Copying 41
Copying 42
Copying 43
Copying 44
Copying 45
Copying 46
Copying 47
Copying 48
Copying 49
Copying 50
Copying 51
Copying 52
Copying 53
Copying 54
Copying 55
Copying 56
Copying 57
Copying 58
Copying 59
Copying 60
Copying 61
Copying 62
Copying 63
Copying 64
Copying 65
Copying 66
Copying 67
Copying 68
Copying 69
Copying 70
Copying 71
Copying 72
Copying 73
Copying 74
Copying 75
Copying 76
Copying 77
Copying 78
Copying 79
Copying 80
Copying 81
Copying 82
Copying 83
Copying 84
Copying 85
Copying 86
Copying 87
Copying 88
Copying 89
Copying 90
Copying 91
Copying 92
Copying 93
Copying 94
Copying 95
Copying 96
Copying 97
Copying 98
Copying 99
Copying 100

```

If you look at the contents of these directories, you'll see that they are
symbolic links to the same files we started to track. 

```console
$ ls -l data/train/
? 1
ls: data/train/: No such file or directory

```
