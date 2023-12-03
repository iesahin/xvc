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
  Time (abs ≡):         31.0 ms               [User: 11.2 ms, System: 17.1 ms]
 

$ hyperfine -r 1 'dvc init ; git add .dvc/ .dvcignore ; git commit -m "Init DVC"'
Benchmark 1: dvc init ; git add .dvc/ .dvcignore ; git commit -m "Init DVC"
  Time (abs ≡):        319.9 ms               [User: 212.4 ms, System: 78.9 ms]
 

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

```console
$ hyperfine -r 1 'xvc file track data/data/*.jpg'
Benchmark 1: xvc file track data/data/*.jpg
  Time (abs ≡):        31.544 s               [User: 30.315 s, System: 12.136 s]
 

$ hyperfine -r 1 'dvc add data/data/*.jpg ; git add data/data/*.dvc ; git commit -m "Added data/data/ to DVC"'
Benchmark 1: dvc add data/data/*.jpg ; git add data/data/*.dvc ; git commit -m "Added data/data/ to DVC"
  Time (abs ≡):        221.587 s               [User: 154.333 s, System: 42.062 s]
 

$ git status -s
?? chinese_mnist.zip
?? data/chinese_mnist.csv

```

## Checkout 15K files

```console
$ rm -rf data/data

$ hyperfine -r 1 'xvc file recheck data/data/'
Benchmark 1: xvc file recheck data/data/
  Time (abs ≡):         8.948 s               [User: 8.761 s, System: 2.652 s]
 

$ rm -rf data/data

$ hyperfine -r 1 --show-output 'git checkout data/data ; for f in $(ls -1 data/data/*.dvc) ; dvc checkout "${f}"'
? 1
Benchmark 1: git checkout data/data ; for f in $(ls -1 data/data/*.dvc) ; dvc checkout "${f}"
sh: -c: line 0: syntax error near unexpected token `dvc'
sh: -c: line 0: `git checkout data/data ; for f in $(ls -1 data/data/*.dvc) ; dvc checkout "${f}"'
Error: Command terminated with non-zero exit code: 2. Use the '-i'/'--ignore-failure' option if you want to ignore this. Alternatively, use the '--show-output' option to debug what went wrong.

```

## Directory with 100K Small Files 

```console,ignore
$ zsh -cl 'mkdir small-files ; for i in {1..100000} ; do echo "data-${RANDOM} ${RANDOM} ${RANDOM}" > small-files/file-${i}.txt ; done'

$ hyperfine -r 1 'xvc file track small-files/'
Benchmark 1: xvc file track small-files/
  Time (abs ≡):        46.030 s               [User: 6.028 s, System: 90.737 s]
 

$ hyperfine -r 1 'dvc add small-files/ ; git add small-files.dvc ; git commit -m "Added small-files/ to DVC"'
? interrupted
Benchmark 1: dvc add small-files/ ; git add small-files.dvc ; git commit -m "Added small-files/ to DVC"

$ git status -s
 M .gitignore
?? chinese_mnist.zip
?? data/
```

