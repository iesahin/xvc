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
  Time (abs ≡):         36.3 ms               [User: 11.7 ms, System: 18.6 ms]
 

$ hyperfine -r 1 'dvc init ; git add .dvc/ .dvcignore ; git commit -m "Init DVC"'
Benchmark 1: dvc init ; git add .dvc/ .dvcignore ; git commit -m "Init DVC"
  Time (abs ≡):        295.4 ms               [User: 211.9 ms, System: 77.4 ms]
 

$ git status -s
?? chinese_mnist.zip

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

Xvc commits the changed metafiles automatically unless otherwise specified in the options. In the DVC command below, we also commit `*.dvc` files.

```console,ignore
$ hyperfine -r 1 'xvc file track data/data/*.jpg'
Benchmark 1: xvc file track data/data/*.jpg
  Time (abs ≡):        33.247 s               [User: 31.723 s, System: 12.490 s]
 

$ hyperfine -r 1 'dvc add data/data/*.jpg ; git add data/data/*.dvc ; git commit -m "Added data/data/ to DVC"'
Benchmark 1: dvc add data/data/*.jpg ; git add data/data/*.dvc ; git commit -m "Added data/data/ to DVC"
  Time (abs ≡):        216.667 s               [User: 156.188 s, System: 41.469 s]
 

$ git status -s
?? chinese_mnist.zip
?? data/chinese_mnist.csv

```

## 1M Small Files Performance

```console
$ zsh -cl 'mkdir small-files ; for i in {1..1000000} ; do echo "data-${RANDOM} ${RANDOM} ${RANDOM}" > small-files/file-${i}.txt ; done'

$ hyperfine -r 1 'xvc file track file-*.txt'
? 1
Benchmark 1: xvc file track small-files/
Error: Command terminated with non-zero exit code: 126. Use the '-i'/'--ignore-failure' option if you want to ignore this. Alternatively, use the '--show-output' option to debug what went wrong.

$ hyperfine -r 1 'dvc add file-*.txt ; git add file-*.dvc ; git commit -m "Added small-files/ to DVC"'
? 1
Benchmark 1: dvc add small-files/ ; git add small-files/ ; git commit -m "Added small-files/ to DVC"
Error: Command terminated with non-zero exit code: 1. Use the '-i'/'--ignore-failure' option if you want to ignore this. Alternatively, use the '--show-output' option to debug what went wrong.

$ git status -s
?? chinese_mnist.zip
?? data/

