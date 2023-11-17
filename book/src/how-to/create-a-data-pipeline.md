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
lrwxr-xr-x  1 iex  staff  191 Nov 17 23:29 chinese_mnist.zip -> [CWD]/.xvc/b3/b24/2c9/422f91b804ea3008bc0bc025e97bf50c1d902ae7a0f13588b84f59023d/0.zip

```

The long directory name is the BLAKE-3 hash of the data file.

As we'll work with the file contents, let's unzip the data file.

```console
$ unzip -q chinese_mnist.zip

$ ls -l
total 0
lrwxr-xr-x  1 iex  staff  191 Nov 17 23:29 chinese_mnist.zip -> [CWD]/.xvc/b3/b24/2c9/422f91b804ea3008bc0bc025e97bf50c1d902ae7a0f13588b84f59023d/0.zip
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
SS         191 2023-11-17 20:29:58 3a714d65          data/data/input_9_9_9.jpg
SS         191 2023-11-17 20:29:58 9ffccc4d          data/data/input_9_9_8.jpg
SS         191 2023-11-17 20:29:59 5d6312a4          data/data/input_9_9_7.jpg
SS         191 2023-11-17 20:29:59 7a0ddb0e          data/data/input_9_9_6.jpg
SS         191 2023-11-17 20:29:58 2047d7f3          data/data/input_9_9_5.jpg
SS         191 2023-11-17 20:29:59 10fcf309          data/data/input_9_9_4.jpg
SS         191 2023-11-17 20:29:58 0bdcd918          data/data/input_9_9_3.jpg
SS         191 2023-11-17 20:29:58 aebcbc03          data/data/input_9_9_2.jpg
SS         191 2023-11-17 20:29:59 38abd173          data/data/input_9_9_15.jpg
SS         191 2023-11-17 20:29:57 7c6a9003          data/data/input_9_9_14.jpg
SS         191 2023-11-17 20:29:58 a9f04ad9          data/data/input_9_9_13.jpg
SS         191 2023-11-17 20:29:59 2d372f95          data/data/input_9_9_12.jpg
SS         191 2023-11-17 20:29:58 8fe799b4          data/data/input_9_9_11.jpg
SS         191 2023-11-17 20:29:58 ee35e5d5          data/data/input_9_9_10.jpg
SS         191 2023-11-17 20:29:58 7576894f          data/data/input_9_9_1.jpg
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



```console 
$ mkdir data/train
$ zsh -cl 'for p in {1..60} ; do xvc file copy \\\'data/data/input_${p}_*\\\' data/train/ ; done'
? 1
zsh:1: no matches found: /data/data/input_1_* data/train/
zsh:1: no matches found: /data/data/input_2_* data/train/
zsh:1: no matches found: /data/data/input_3_* data/train/
zsh:1: no matches found: /data/data/input_4_* data/train/
zsh:1: no matches found: /data/data/input_5_* data/train/
zsh:1: no matches found: /data/data/input_6_* data/train/
zsh:1: no matches found: /data/data/input_7_* data/train/
zsh:1: no matches found: /data/data/input_8_* data/train/
zsh:1: no matches found: /data/data/input_9_* data/train/
zsh:1: no matches found: /data/data/input_10_* data/train/
zsh:1: no matches found: /data/data/input_11_* data/train/
zsh:1: no matches found: /data/data/input_12_* data/train/
zsh:1: no matches found: /data/data/input_13_* data/train/
zsh:1: no matches found: /data/data/input_14_* data/train/
zsh:1: no matches found: /data/data/input_15_* data/train/
zsh:1: no matches found: /data/data/input_16_* data/train/
zsh:1: no matches found: /data/data/input_17_* data/train/
zsh:1: no matches found: /data/data/input_18_* data/train/
zsh:1: no matches found: /data/data/input_19_* data/train/
zsh:1: no matches found: /data/data/input_20_* data/train/
zsh:1: no matches found: /data/data/input_21_* data/train/
zsh:1: no matches found: /data/data/input_22_* data/train/
zsh:1: no matches found: /data/data/input_23_* data/train/
zsh:1: no matches found: /data/data/input_24_* data/train/
zsh:1: no matches found: /data/data/input_25_* data/train/
zsh:1: no matches found: /data/data/input_26_* data/train/
zsh:1: no matches found: /data/data/input_27_* data/train/
zsh:1: no matches found: /data/data/input_28_* data/train/
zsh:1: no matches found: /data/data/input_29_* data/train/
zsh:1: no matches found: /data/data/input_30_* data/train/
zsh:1: no matches found: /data/data/input_31_* data/train/
zsh:1: no matches found: /data/data/input_32_* data/train/
zsh:1: no matches found: /data/data/input_33_* data/train/
zsh:1: no matches found: /data/data/input_34_* data/train/
zsh:1: no matches found: /data/data/input_35_* data/train/
zsh:1: no matches found: /data/data/input_36_* data/train/
zsh:1: no matches found: /data/data/input_37_* data/train/
zsh:1: no matches found: /data/data/input_38_* data/train/
zsh:1: no matches found: /data/data/input_39_* data/train/
zsh:1: no matches found: /data/data/input_40_* data/train/
zsh:1: no matches found: /data/data/input_41_* data/train/
zsh:1: no matches found: /data/data/input_42_* data/train/
zsh:1: no matches found: /data/data/input_43_* data/train/
zsh:1: no matches found: /data/data/input_44_* data/train/
zsh:1: no matches found: /data/data/input_45_* data/train/
zsh:1: no matches found: /data/data/input_46_* data/train/
zsh:1: no matches found: /data/data/input_47_* data/train/
zsh:1: no matches found: /data/data/input_48_* data/train/
zsh:1: no matches found: /data/data/input_49_* data/train/
zsh:1: no matches found: /data/data/input_50_* data/train/
zsh:1: no matches found: /data/data/input_51_* data/train/
zsh:1: no matches found: /data/data/input_52_* data/train/
zsh:1: no matches found: /data/data/input_53_* data/train/
zsh:1: no matches found: /data/data/input_54_* data/train/
zsh:1: no matches found: /data/data/input_55_* data/train/
zsh:1: no matches found: /data/data/input_56_* data/train/
zsh:1: no matches found: /data/data/input_57_* data/train/
zsh:1: no matches found: /data/data/input_58_* data/train/
zsh:1: no matches found: /data/data/input_59_* data/train/
zsh:1: no matches found: /data/data/input_60_* data/train/

```
