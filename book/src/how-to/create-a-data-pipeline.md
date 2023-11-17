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
$ xvc file track chinese_mnist.zip --recheck-as symlink
```

The default [recheck (checkout) method](/ref/xvc-file-recheck.md) is _copy_ that means the file is
duplicated in the workspace as a writable file. We don't need to write on this
data file, we'll only read from it, so we set the recheck type as symlink.

```console
$ ls -l
```


$ xvc pipeline step new -s convert-docx-to-txt --command "./convert-docx-to-txt.zsh" 
```


