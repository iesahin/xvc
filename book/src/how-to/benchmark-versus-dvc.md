# Benchmarking Xvc vs DVC

In this section, we'll write a few tests to see how Xvc and DVC perform in common tasks. This document is planned as reproducible to see the differences in performance. I'll update this time to time to see the differences, and I'll also add more tests.

This is mostly to satisfy my personal curiosity. I don't claim these are scientific experiments that describe the performance in all conditions. 

We'll test the tools in the following scenarios:

- Checking in small files: We'll unzip 15.000 images from Chinese-MNIST dataset and measure the time for `dvc add` and `xvc file track`
- Checking out small files: We'll delete the files we track and recheck / checkout them using `dvc checkout`  and `xvc recheck`
- Pushing/sending the small files we added to S3 
- Pulling/bringing the small files we pushed from S3
- Checking in and out large files: We'll create 100 large files using `xvc-test-helper` and repeat the above tests.
- Running small pipelines: We'll create a pipeline with 10 steps to run simple commands.
- Running medium sized pipelines: We'll create a pipeline with 100 steps to run simple commands.
- Running large pipelines: We'll create a pipeline with 10000 steps to run simple commands. 

## Setup

This document uses the most recent versions of Xvc and DVC. DVC is installed via Homebrew. 

```console
$ dvc --version
3.30.3

$ xvc --version
xvc v0.6.4-alpha.0-294-g20d863a-modified

```

## Init Repositories

Let's start by measuring the performance of initializing repositories. 

```console
$ git init
Initialized empty Git repository in [CWD]/.git/

$ hyperfine -r 1 'xvc init'
Benchmark 1: xvc init
  Time (abs ≡):         31.4 ms               [User: 11.1 ms, System: 16.8 ms]
 

$ hyperfine -r 1 'dvc init ; git add .dvc/ .dvcignore ; git commit -m "Init DVC"'
Benchmark 1: dvc init ; git add .dvc/ .dvcignore ; git commit -m "Init DVC"
  Time (abs ≡):        280.4 ms               [User: 198.7 ms, System: 71.6 ms]
 

$ git status -s
?? chinese_mnist.zip

```

## Unzip the images

```console
$ unzip -q chinese_mnist.zip
$ zsh -cl 'cp -r data/data xvc-data'
$ zsh -cl 'cp -r data/data dvc-data'
$ tree -d
.
├── data
│   └── data
├── dvc-data
└── xvc-data

5 directories

```


## 15K Small Files Performance

Xvc commits the changed metafiles automatically unless otherwise specified in the options. In the DVC command below, we also commit `*.dvc` files.

```console,ignore
$ hyperfine -r 1 'xvc file track xvc-data/'
Benchmark 1: xvc file track xvc-data/
  Time (abs ≡):         3.655 s               [User: 0.931 s, System: 12.339 s]
 

$ hyperfine -r 1 --show-output 'dvc add dvc-data/ '
Benchmark 1: dvc add dvc-data/ 

To track the changes with git, run:

	git add .gitignore dvc-data.dvc

To enable auto staging, run:

	dvc config core.autostage true
  Time (abs ≡):        13.027 s               [User: 4.740 s, System: 6.765 s]
 

$ lsd -l

$ git status -s
 M .gitignore
?? chinese_mnist.zip
?? data/
?? dvc-data.dvc

```

## Checkout a directory with 15K files

```console,ignore
$ rm -rf xvc-data

$ hyperfine -r 1 'xvc file recheck xvc-data/'
Benchmark 1: xvc file recheck xvc-data/
  Time (abs ≡):         2.378 s               [User: 0.438 s, System: 2.152 s]
 

$ rm -rf dvc-data/

$ ls 
chinese_mnist.zip
data
dvc-data.dvc
xvc-data

$ hyperfine -r 1 --show-output 'dvc checkout dvc-data.dvc'
Benchmark 1: dvc checkout dvc-data.dvc
A       dvc-data/
  Time (abs ≡):         4.102 s               [User: 1.399 s, System: 2.155 s]
 

```

## Large File Performance

```console,ignore
$ zsh -cl 'dd if=/dev/urandom of=xvc-large-file bs=1M count=1000'
1000+0 records in
1000+0 records out
1048576000 bytes transferred in 1.669660 secs (628017680 bytes/sec)

$ hyperfine -r 1 'xvc file track xvc-large-file'
Benchmark 1: xvc file track xvc-large-file
  Time (abs ≡):         1.499 s               [User: 0.816 s, System: 0.805 s]
 

$ zsh -cl 'dd if=/dev/urandom of=dvc-large-file bs=1M count=1000'
1000+0 records in
1000+0 records out
1048576000 bytes transferred in 1.446919 secs (724695716 bytes/sec)

$ hyperfine -r 1 --show-output 'dvc add dvc-large-file ; git add dvc-large-file.dvc .gitignore ; git commit -m "Added dvc-large-file to DVC"'
Benchmark 1: dvc add dvc-large-file ; git add dvc-large-file.dvc .gitignore ; git commit -m "Added dvc-large-file to DVC"

To track the changes with git, run:

	git add dvc-large-file.dvc .gitignore

To enable auto staging, run:

	dvc config core.autostage true
[main 72fd199] Added dvc-large-file to DVC
 2 files changed, 6 insertions(+)
 create mode 100644 dvc-large-file.dvc
  Time (abs ≡):         2.153 s               [User: 1.906 s, System: 0.203 s]
 

```

## Commit/Carry-in Large Files

```console,ignore
$ zsh -cl 'dd if=/dev/urandom of=xvc-large-file bs=1M count=1000'
1000+0 records in
1000+0 records out
1048576000 bytes transferred in 1.550065 secs (676472277 bytes/sec)

$ hyperfine -r 1 'xvc file carry-in xvc-large-file'
Benchmark 1: xvc file carry-in xvc-large-file
  Time (abs ≡):         1.024 s               [User: 0.629 s, System: 0.393 s]
 

$ zsh -cl 'dd if=/dev/urandom of=dvc-large-file bs=1M count=1000'
1000+0 records in
1000+0 records out
1048576000 bytes transferred in 1.550363 secs (676342250 bytes/sec)

$ hyperfine -r 1 --show-output 'dvc add dvc-large-file ; git add dvc-large-file.dvc ; git commit -m "Added dvc-large-file to DVC"'
Benchmark 1: dvc add dvc-large-file ; git add dvc-large-file.dvc ; git commit -m "Added dvc-large-file to DVC"

To track the changes with git, run:

	git add dvc-large-file.dvc

To enable auto staging, run:

	dvc config core.autostage true
[main c74d783] Added dvc-large-file to DVC
 1 file changed, 1 insertion(+), 1 deletion(-)
  Time (abs ≡):         2.098 s               [User: 1.903 s, System: 0.189 s]
 

```

## Pipeline with 10 Steps

Pipeline steps will depend on the following files. 

```console
$ xvc-test-helper create-directory-tree --directories 1 --files 10  --root pipeline-10

$ tree pipeline-10
pipeline-10
└── dir-0001
    ├── file-0001.bin
    ├── file-0002.bin
    ├── file-0003.bin
    ├── file-0004.bin
    ├── file-0005.bin
    ├── file-0006.bin
    ├── file-0007.bin
    ├── file-0008.bin
    ├── file-0009.bin
    └── file-0010.bin

2 directories, 10 files

```

Let's create 10 DVC stages to depend on these files:

```
$ zsh -cl "for f in pipeline-10/dir-0001/* ; do dvc stage add -q -n ${f:r:t} -d ${f} 'sha1sum $f'; done"

$ dvc stage list
file-0001  Depends on pipeline-10/dir-0001/file-0001.bin
file-0002  Depends on pipeline-10/dir-0001/file-0002.bin
file-0003  Depends on pipeline-10/dir-0001/file-0003.bin
file-0004  Depends on pipeline-10/dir-0001/file-0004.bin
file-0005  Depends on pipeline-10/dir-0001/file-0005.bin
file-0006  Depends on pipeline-10/dir-0001/file-0006.bin
file-0007  Depends on pipeline-10/dir-0001/file-0007.bin
file-0008  Depends on pipeline-10/dir-0001/file-0008.bin
file-0009  Depends on pipeline-10/dir-0001/file-0009.bin
file-0010  Depends on pipeline-10/dir-0001/file-0010.bin

```

Run the DVC pipeline

```console
$ hyperfine -r 1 "dvc repro"
Benchmark 1: dvc repro
  Time (abs ≡):        617.4 ms               [User: 438.2 ms, System: 162.5 ms]
 

```

Running without changed the dependencies
```console
$ hyperfine -M 5 "dvc repro"
Benchmark 1: dvc repro
  Time (mean ± σ):     433.8 ms ±   9.3 ms    [User: 327.6 ms, System: 99.9 ms]
  Range (min … max):   421.8 ms … 444.8 ms    5 runs
 

```
```console
$ zsh -cl "for f in pipeline-10/dir-0001/* ; do xvc pipeline step new -s ${f:r:t} --command 'sha1sum $f' ; xvc pipeline step dependency -s ${f:r:t} --file ${f} ; done"

$ hyperfine -r 1 "xvc pipeline run"
Benchmark 1: xvc pipeline run
  Time (abs ≡):        220.1 ms               [User: 53.1 ms, System: 214.5 ms]
 

```

```console
$ hyperfine -M 5 "xvc pipeline run"
Benchmark 1: xvc pipeline run
  Time (mean ± σ):     174.9 ms ±   3.3 ms    [User: 34.4 ms, System: 143.1 ms]
  Range (min … max):   171.4 ms … 179.9 ms    5 runs
 

```



## Pipeline with 100 Steps

Pipeline steps will depend on the following files. 

```console
$ xvc-test-helper create-directory-tree --directories 1 --files 100 --root pipeline-100

$ tree -d pipeline-100
pipeline-100
└── dir-0001

2 directories

$ rm -f dvc.yaml

$ zsh -cl "for f in pipeline-100/dir-0001/* ; do dvc stage add -q -n s-${RANDOM} -d ${f} 'sha1sum $f'; done"


$ hyperfine -r 1 "dvc repro"
Benchmark 1: dvc repro
  Time (abs ≡):         9.389 s               [User: 8.350 s, System: 0.904 s]
 

$ hyperfine -M 5 "dvc repro"
Benchmark 1: dvc repro
  Time (mean ± σ):     639.2 ms ±  30.6 ms    [User: 467.7 ms, System: 163.2 ms]
  Range (min … max):   612.6 ms … 690.9 ms    5 runs
 
```

Let's create 100 Xvc steps to depend on the same files. 

```console
$ xvc pipeline new --pipeline-name p100

$ zsh -cl "for f in pipeline-100/dir-0001/* ; do xvc pipeline -p p100 step new -s ${f:r:t} --command 'sha1sum $f' ; xvc pipeline -p p100 step dependency -s ${f:r:t} --file ${f} ; done"

$ hyperfine -r 1 --show-output "xvc pipeline -p p100 run" 
Benchmark 1: xvc pipeline -p p100 run
  Time (abs ≡):        196.5 ms               [User: 39.5 ms, System: 159.0 ms]
 

$ hyperfine -M 5 "xvc pipeline -p p100 run"
Benchmark 1: xvc pipeline -p p100 run
  Time (mean ± σ):     198.2 ms ±   2.4 ms    [User: 39.7 ms, System: 163.0 ms]
  Range (min … max):   194.2 ms … 200.2 ms    5 runs
 

```

Note that the first run of the commands is drastically different. DVC runs all stages sequentially, in around 9.3 seconds while Xvc runs them in parallel in 0.2 seconds. Let's also measure the average run time of a `sha1sum` command to consider how much of these passes in actual commands. 

```
$ hyperfine 'sha1sum pipeline-100/dir-0001/file-0001.bin'
```

## Pipeline with 10000 Steps

In this case we'll just measure the run times of 10000 `ls` commands. 

```console
$ rm -f dvc.yaml

$ zsh -cl "for i in {1..10000}; do dvc stage add -q -n s-${i} 'ls'; done"


$ hyperfine -r 1 "dvc repro"
Benchmark 1: dvc repro
  Time (abs ≡):         9.389 s               [User: 8.350 s, System: 0.904 s]
 

$ hyperfine -M 5 "dvc repro"

```


```console

$ xvc pipeline new --pipeline-name p10000

$ zsh -cl "for i in {1..10000} ; do xvc pipeline -p p10000 step new -s s-${i} --command 'ls' ; done"

$ hyperfine -r 1 --show-output "xvc pipeline -p p10000 run" 
Benchmark 1: xvc pipeline -p p100 run
  Time (abs ≡):        196.5 ms               [User: 39.5 ms, System: 159.0 ms]
 

$ hyperfine -M 5 "xvc pipeline -p p10000 run"
Benchmark 1: xvc pipeline -p p100 run
  Time (mean ± σ):     198.2 ms ±   2.4 ms    [User: 39.7 ms, System: 163.0 ms]
  Range (min … max):   194.2 ms … 200.2 ms    5 runs
 

```
