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
total 21080
-rw-r--r--  1 iex  staff  10792680 Nov 17 19:46 chinese_mnist.zip

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
total 0
lrwxr-xr-x  1 iex  staff  192 Nov 17 23:34 chinese_mnist.zip -> [CWD]/.xvc/b3/b24/2c9/422f91b804ea3008bc0bc025e97bf50c1d902ae7a0f13588b84f59023d/0.zip

```

The long directory name is the BLAKE-3 hash of the data file.

As we'll work with the file contents, let's unzip the data file.

```console
$ unzip -q chinese_mnist.zip

$ ls -l
total 0
lrwxr-xr-x  1 iex  staff  192 Nov 17 23:34 chinese_mnist.zip -> [CWD]/.xvc/b3/b24/2c9/422f91b804ea3008bc0bc025e97bf50c1d902ae7a0f13588b84f59023d/0.zip
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
SS         192 2023-11-17 20:34:52 3a714d65          data/data/input_9_9_9.jpg
SS         192 2023-11-17 20:34:51 9ffccc4d          data/data/input_9_9_8.jpg
SS         192 2023-11-17 20:34:52 5d6312a4          data/data/input_9_9_7.jpg
SS         192 2023-11-17 20:34:51 7a0ddb0e          data/data/input_9_9_6.jpg
SS         192 2023-11-17 20:34:52 2047d7f3          data/data/input_9_9_5.jpg
SS         192 2023-11-17 20:34:52 10fcf309          data/data/input_9_9_4.jpg
SS         192 2023-11-17 20:34:52 0bdcd918          data/data/input_9_9_3.jpg
SS         192 2023-11-17 20:34:52 aebcbc03          data/data/input_9_9_2.jpg
SS         192 2023-11-17 20:34:51 38abd173          data/data/input_9_9_15.jpg
SS         192 2023-11-17 20:34:52 7c6a9003          data/data/input_9_9_14.jpg
SS         192 2023-11-17 20:34:52 a9f04ad9          data/data/input_9_9_13.jpg
SS         192 2023-11-17 20:34:53 2d372f95          data/data/input_9_9_12.jpg
SS         192 2023-11-17 20:34:51 8fe799b4          data/data/input_9_9_11.jpg
SS         192 2023-11-17 20:34:51 ee35e5d5          data/data/input_9_9_10.jpg
SS         192 2023-11-17 20:34:50 7576894f          data/data/input_9_9_1.jpg
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
$ zsh -cl 'for p in {1..60} ; do xvc file copy data/data/input_${p}_\* data/train/ ; done'
$ zsh -cl 'for p in {61..80} ; do xvc file copy data/data/input_${p}_\* data/validate/ ; done'
$ zsh -cl 'for p in {81..100} ; do xvc file copy data/data/input_${p}_\* data/test/ ; done'

```

If you look at the contents of these directories, you'll see that they are
symbolic links to the same files we started to track. 

```

$ ls -l data/train/input_9_9*
```
