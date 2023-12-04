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
  Time (abs ≡):         30.7 ms               [User: 11.0 ms, System: 17.5 ms]
 

$ hyperfine -r 1 'dvc init ; git add .dvc/ .dvcignore ; git commit -m "Init DVC"'
Benchmark 1: dvc init ; git add .dvc/ .dvcignore ; git commit -m "Init DVC"
  Time (abs ≡):        283.5 ms               [User: 201.9 ms, System: 72.6 ms]
 

$ git status -s
?? chinese_mnist.zip

```

## Unzip the images

```console
$ unzip -q chinese_mnist.zip
$ cp -r data/data xvc-data
$ cp -r data/data dvc-data
$ tree -d
```


## 15K Small Files Performance

Xvc commits the changed metafiles automatically unless otherwise specified in the options. In the DVC command below, we also commit `*.dvc` files.

```console
$ hyperfine -r 1 'xvc file track xvc-data/'
Benchmark 1: xvc file track xvc-data/
  Time (abs ≡):        297.3 ms               [User: 77.5 ms, System: 219.1 ms]
 

$ hyperfine -r 1 --show-output 'dvc add dvc-data/ '


$ lsd -l

? 1

nothing added to commit but untracked files present (use "git add" to track)
Error: Command terminated with non-zero exit code: 1. Use the '-i'/'--ignore-failure' option if you want to ignore this. Alternatively, use the '--show-output' option to debug what went wrong.

$ git status -s
?? chinese_mnist.zip
?? data/

```

## Checkout 15K files

```console
$ rm -rf xvc-data

$ hyperfine -r 1 'xvc file recheck xvc-data/'
Benchmark 1: xvc file recheck xvc-data/
  Time (abs ≡):        103.5 ms               [User: 20.8 ms, System: 80.7 ms]
 

$ rm -rf dvc-data/

$ ls 
chinese_mnist.zip
data

$ hyperfine -r 1 --show-output 'dvc checkout dvc-data.dvc'
? 1
Benchmark 1: dvc checkout dvc-data.dvc
ERROR: Did you mean `git checkout dvc-data.dvc`?: '[CWD]/dvc-data.dvc' does not exist
Error: Command terminated with non-zero exit code: 255. Use the '-i'/'--ignore-failure' option if you want to ignore this. Alternatively, use the '--show-output' option to debug what went wrong.

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

