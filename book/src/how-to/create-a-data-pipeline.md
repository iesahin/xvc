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
-rwxr-xr-x  1 iex  staff       261 Nov 17 23:52 create-subsets.zsh

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
lrwxr-xr-x  1 iex  staff  192 Nov 18 00:04 chinese_mnist.zip -> [CWD]/.xvc/b3/b24/2c9/422f91b804ea3008bc0bc025e97bf50c1d902ae7a0f13588b84f59023d/0.zip
-rwxr-xr-x  1 iex  staff  261 Nov 17 23:52 create-subsets.zsh

```

The long directory name is the BLAKE-3 hash of the data file.

As we'll work with the file contents, let's unzip the data file.

```console
$ unzip -q chinese_mnist.zip

$ ls -l
total 8
lrwxr-xr-x  1 iex  staff  192 Nov 18 00:04 chinese_mnist.zip -> [CWD]/.xvc/b3/b24/2c9/422f91b804ea3008bc0bc025e97bf50c1d902ae7a0f13588b84f59023d/0.zip
-rwxr-xr-x  1 iex  staff  261 Nov 17 23:52 create-subsets.zsh
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
SS         192 2023-11-17 21:04:20 3a714d65          data/data/input_9_9_9.jpg
SS         192 2023-11-17 21:04:17 9ffccc4d          data/data/input_9_9_8.jpg
SS         192 2023-11-17 21:04:19 5d6312a4          data/data/input_9_9_7.jpg
SS         192 2023-11-17 21:04:18 7a0ddb0e          data/data/input_9_9_6.jpg
SS         192 2023-11-17 21:04:18 2047d7f3          data/data/input_9_9_5.jpg
SS         192 2023-11-17 21:04:18 10fcf309          data/data/input_9_9_4.jpg
SS         192 2023-11-17 21:04:19 0bdcd918          data/data/input_9_9_3.jpg
SS         192 2023-11-17 21:04:18 aebcbc03          data/data/input_9_9_2.jpg
SS         192 2023-11-17 21:04:19 38abd173          data/data/input_9_9_15.jpg
SS         192 2023-11-17 21:04:18 7c6a9003          data/data/input_9_9_14.jpg
SS         192 2023-11-17 21:04:18 a9f04ad9          data/data/input_9_9_13.jpg
SS         192 2023-11-17 21:04:18 2d372f95          data/data/input_9_9_12.jpg
SS         192 2023-11-17 21:04:19 8fe799b4          data/data/input_9_9_11.jpg
SS         192 2023-11-17 21:04:19 ee35e5d5          data/data/input_9_9_10.jpg
SS         192 2023-11-17 21:04:17 7576894f          data/data/input_9_9_1.jpg
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

for p in {1..60} ; do xvc file copy data/data/input_${p}_* data/train/ ; done

for p in {61..80} ; do xvc file copy data/data/input_${p}_* data/validate/ ; done

for p in {81..100} ; do xvc file copy data/data/input_${p}_* data/test/ ; done

```

```console
$ zsh create-subsets.zsh
? 2
error: unexpected argument 'data/data/input_1_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_2_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_3_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_4_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_5_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_6_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_7_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_8_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_9_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_10_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_11_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_12_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_13_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_14_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_15_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_16_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_17_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_18_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_19_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_20_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_21_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_22_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_23_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_24_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_25_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_26_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_27_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_28_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_29_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_30_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_31_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_32_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_33_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_34_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_35_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_36_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_37_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_38_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_39_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_40_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_41_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_42_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_43_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_44_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_45_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_46_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_47_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_48_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_49_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_50_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_51_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_52_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_53_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_54_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_55_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_56_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_57_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_58_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_59_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_60_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_61_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_62_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_63_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_64_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_65_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_66_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_67_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_68_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_69_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_70_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_71_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_72_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_73_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_74_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_75_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_76_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_77_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_78_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_79_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_80_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_81_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_82_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_83_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_84_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_85_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_86_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_87_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_88_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_89_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_90_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_91_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_92_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_93_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_94_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_95_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_96_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_97_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_98_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_99_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.
error: unexpected argument 'data/data/input_100_10_11.jpg' found

Usage: xvc file copy [OPTIONS] <SOURCE> <DESTINATION>

For more information, try '--help'.

```

If you look at the contents of these directories, you'll see that they are
symbolic links to the same files we started to track. 

```console
$ ls -l data/train/
? 1
ls: data/train/: No such file or directory

```
