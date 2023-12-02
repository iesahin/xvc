# Benchmarking Xvc vs DVC

In this section, we'll write a few tests to see how Xvc and DVC perform in common tasks. This document is planned as reproducible to see the differences in performance. I'll update this time to time to see the differences, and I'll also add more tests.

This is mostly to satisfy my personal curiosity. I don't claim these are scientific experiments that describe the performance in all conditions. 

We'll test the tools in the following scenarios:

- Checking in small files: We'll unzip 15.000 images from Chinese-MNIST dataset and measure the time for `dvc add` and `xvc file track`
- Checking out small files: We'll delete the files we track and recheck / checkout them using `dvc checkout`  and `xvc recheck`
- Pushing/sending the small files we added to S3 
- Pulling/bringing the small files we pushed from S3
- Checking in and out large files: We'll create 100 large files using `xvc-test-helper` and repeat the above tests.
- Running small pipelines: We'll create a pipeline with 5 steps to run simple commands.
- Running large pipelines: We'll create a pipeline with 10000 steps to run simple commands. 

## Setup

This document uses the most recent versions of Xvc and DVC. DVC is installed via Homebrew. 

```console
$ dvc --version
3.30.3

$ xvc --version
xvc 0.6.4-alpha.0

```

## Init Repositories

Let's start by measuring the performance of initializing repositories. 

```console
$ git init
Initialized empty Git repository in [CWD]/.git/

$ hyperfine -r 1 'xvc init'
Benchmark 1: xvc init
  Time (abs ≡):         32.7 ms               [User: 11.1 ms, System: 17.2 ms]
 

$ hyperfine -r 1 'dvc init'
Benchmark 1: dvc init
  Time (abs ≡):        270.1 ms               [User: 192.9 ms, System: 63.1 ms]
 
$ git status -s
```

## Unzip the images

```console
$ unzip -q chinese_mnist.zip
$ tree -d 
.
└── data
    └── data

3 directories

```


## 15K Small Files Performance

```console
$ hyperfine -r 1 'xvc file track data/data/*.jpg'
Benchmark 1: xvc file track data/
  Time (abs ≡):         9.600 s               [User: 8.985 s, System: 11.442 s]
 
$ hyperfine -r 1 'dvc add data/data/*.jpg'
Benchmark 1: dvc add data/
  Time (abs ≡):        12.595 s               [User: 4.701 s, System: 6.392 s]

$ git status -s


```

