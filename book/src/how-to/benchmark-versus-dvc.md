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
xvc v0.6.4-alpha.0-293-g2e977b1-modified

```

## Init Repositories

Let's start by measuring the performance of initializing repositories. 

```console
$ git init
Initialized empty Git repository in [CWD]/.git/

$ hyperfine -r 1 'xvc init'
Benchmark 1: xvc init
  Time (abs ≡):         43.2 ms               [User: 11.0 ms, System: 19.6 ms]
 

$ hyperfine -r 1 'dvc init ; git add .dvc/ .dvcignore ; git commit -m "Init DVC"'
Benchmark 1: dvc init ; git add .dvc/ .dvcignore ; git commit -m "Init DVC"
  Time (abs ≡):        343.8 ms               [User: 207.2 ms, System: 79.3 ms]
 

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
  Time (abs ≡):        605.3 ms               [User: 432.6 ms, System: 154.5 ms]
 

```

Running without changed the dependencies
```console
$ hyperfine -M 5 "dvc repro"
Benchmark 1: dvc repro
  Time (mean ± σ):     433.4 ms ±   5.1 ms    [User: 328.5 ms, System: 99.8 ms]
  Range (min … max):   429.4 ms … 440.9 ms    5 runs
 
  Warning: Statistical outliers were detected. Consider re-running this benchmark on a quiet system without any interferences from other programs. It might help to use the '--warmup' or '--prepare' options.
 

```
```console
$ zsh -cl "for f in pipeline-10/dir-0001/* ; do xvc pipeline step new -s ${f:r:t} --command 'sha1sum $f' ; xvc pipeline step dependency -s ${f:r:t} --file ${f} ; done"

$ hyperfine -r 1 "xvc pipeline run"
Benchmark 1: xvc pipeline run
  Time (abs ≡):        225.3 ms               [User: 53.7 ms, System: 225.5 ms]
 

```

```console
$ hyperfine -M 5 "xvc pipeline run"
Benchmark 1: xvc pipeline run
  Time (mean ± σ):     177.4 ms ±   2.3 ms    [User: 34.8 ms, System: 146.1 ms]
  Range (min … max):   173.7 ms … 179.5 ms    5 runs
 

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

$ dvc stage list
s-27956  Depends on pipeline-100/dir-0001/file-0001.bin
s-8141   Depends on pipeline-100/dir-0001/file-0002.bin
s-32412  Depends on pipeline-100/dir-0001/file-0003.bin
s-15431  Depends on pipeline-100/dir-0001/file-0004.bin
s-243    Depends on pipeline-100/dir-0001/file-0005.bin
s-4878   Depends on pipeline-100/dir-0001/file-0006.bin
s-7575   Depends on pipeline-100/dir-0001/file-0007.bin
s-19721  Depends on pipeline-100/dir-0001/file-0008.bin
s-14627  Depends on pipeline-100/dir-0001/file-0009.bin
s-17940  Depends on pipeline-100/dir-0001/file-0010.bin
s-28609  Depends on pipeline-100/dir-0001/file-0011.bin
s-29036  Depends on pipeline-100/dir-0001/file-0012.bin
s-32218  Depends on pipeline-100/dir-0001/file-0013.bin
s-32067  Depends on pipeline-100/dir-0001/file-0014.bin
s-23450  Depends on pipeline-100/dir-0001/file-0015.bin
s-25623  Depends on pipeline-100/dir-0001/file-0016.bin
s-9139   Depends on pipeline-100/dir-0001/file-0017.bin
s-26219  Depends on pipeline-100/dir-0001/file-0018.bin
s-5769   Depends on pipeline-100/dir-0001/file-0019.bin
s-10164  Depends on pipeline-100/dir-0001/file-0020.bin
s-15101  Depends on pipeline-100/dir-0001/file-0021.bin
s-21310  Depends on pipeline-100/dir-0001/file-0022.bin
s-4836   Depends on pipeline-100/dir-0001/file-0023.bin
s-24924  Depends on pipeline-100/dir-0001/file-0024.bin
s-3125   Depends on pipeline-100/dir-0001/file-0025.bin
s-973    Depends on pipeline-100/dir-0001/file-0026.bin
s-3984   Depends on pipeline-100/dir-0001/file-0027.bin
s-17847  Depends on pipeline-100/dir-0001/file-0028.bin
s-416    Depends on pipeline-100/dir-0001/file-0029.bin
s-21786  Depends on pipeline-100/dir-0001/file-0030.bin
s-21920  Depends on pipeline-100/dir-0001/file-0031.bin
s-4185   Depends on pipeline-100/dir-0001/file-0032.bin
s-22247  Depends on pipeline-100/dir-0001/file-0033.bin
s-642    Depends on pipeline-100/dir-0001/file-0034.bin
s-22992  Depends on pipeline-100/dir-0001/file-0035.bin
s-2757   Depends on pipeline-100/dir-0001/file-0036.bin
s-18515  Depends on pipeline-100/dir-0001/file-0037.bin
s-31121  Depends on pipeline-100/dir-0001/file-0038.bin
s-10746  Depends on pipeline-100/dir-0001/file-0039.bin
s-25339  Depends on pipeline-100/dir-0001/file-0040.bin
s-28328  Depends on pipeline-100/dir-0001/file-0041.bin
s-30661  Depends on pipeline-100/dir-0001/file-0042.bin
s-23231  Depends on pipeline-100/dir-0001/file-0043.bin
s-26171  Depends on pipeline-100/dir-0001/file-0044.bin
s-24788  Depends on pipeline-100/dir-0001/file-0045.bin
s-1630   Depends on pipeline-100/dir-0001/file-0046.bin
s-13195  Depends on pipeline-100/dir-0001/file-0047.bin
s-8788   Depends on pipeline-100/dir-0001/file-0048.bin
s-26097  Depends on pipeline-100/dir-0001/file-0049.bin
s-28712  Depends on pipeline-100/dir-0001/file-0050.bin
s-29334  Depends on pipeline-100/dir-0001/file-0051.bin
s-23136  Depends on pipeline-100/dir-0001/file-0052.bin
s-28016  Depends on pipeline-100/dir-0001/file-0053.bin
s-26592  Depends on pipeline-100/dir-0001/file-0054.bin
s-10996  Depends on pipeline-100/dir-0001/file-0055.bin
s-9281   Depends on pipeline-100/dir-0001/file-0056.bin
s-17202  Depends on pipeline-100/dir-0001/file-0057.bin
s-8703   Depends on pipeline-100/dir-0001/file-0058.bin
s-9474   Depends on pipeline-100/dir-0001/file-0059.bin
s-12038  Depends on pipeline-100/dir-0001/file-0060.bin
s-17300  Depends on pipeline-100/dir-0001/file-0061.bin
s-14237  Depends on pipeline-100/dir-0001/file-0062.bin
s-18154  Depends on pipeline-100/dir-0001/file-0063.bin
s-26394  Depends on pipeline-100/dir-0001/file-0064.bin
s-3769   Depends on pipeline-100/dir-0001/file-0065.bin
s-5222   Depends on pipeline-100/dir-0001/file-0066.bin
s-20079  Depends on pipeline-100/dir-0001/file-0067.bin
s-29424  Depends on pipeline-100/dir-0001/file-0068.bin
s-5315   Depends on pipeline-100/dir-0001/file-0069.bin
s-14245  Depends on pipeline-100/dir-0001/file-0070.bin
s-21734  Depends on pipeline-100/dir-0001/file-0071.bin
s-23631  Depends on pipeline-100/dir-0001/file-0072.bin
s-22591  Depends on pipeline-100/dir-0001/file-0073.bin
s-13472  Depends on pipeline-100/dir-0001/file-0074.bin
s-1663   Depends on pipeline-100/dir-0001/file-0075.bin
s-13721  Depends on pipeline-100/dir-0001/file-0076.bin
s-30735  Depends on pipeline-100/dir-0001/file-0077.bin
s-11610  Depends on pipeline-100/dir-0001/file-0078.bin
s-7902   Depends on pipeline-100/dir-0001/file-0079.bin
s-7411   Depends on pipeline-100/dir-0001/file-0080.bin
s-14916  Depends on pipeline-100/dir-0001/file-0081.bin
s-32024  Depends on pipeline-100/dir-0001/file-0082.bin
s-13295  Depends on pipeline-100/dir-0001/file-0083.bin
s-16560  Depends on pipeline-100/dir-0001/file-0084.bin
s-31766  Depends on pipeline-100/dir-0001/file-0085.bin
s-14445  Depends on pipeline-100/dir-0001/file-0086.bin
s-7381   Depends on pipeline-100/dir-0001/file-0087.bin
s-5205   Depends on pipeline-100/dir-0001/file-0088.bin
s-27612  Depends on pipeline-100/dir-0001/file-0089.bin
s-16190  Depends on pipeline-100/dir-0001/file-0090.bin
s-13652  Depends on pipeline-100/dir-0001/file-0091.bin
s-16974  Depends on pipeline-100/dir-0001/file-0092.bin
s-5104   Depends on pipeline-100/dir-0001/file-0093.bin
s-4010   Depends on pipeline-100/dir-0001/file-0094.bin
s-4952   Depends on pipeline-100/dir-0001/file-0095.bin
s-3719   Depends on pipeline-100/dir-0001/file-0096.bin
s-280    Depends on pipeline-100/dir-0001/file-0097.bin
s-21143  Depends on pipeline-100/dir-0001/file-0098.bin
s-28801  Depends on pipeline-100/dir-0001/file-0099.bin
s-20959  Depends on pipeline-100/dir-0001/file-0100.bin

$ hyperfine -r 1 "dvc repro"
Benchmark 1: dvc repro
  Time (abs ≡):         9.505 s               [User: 8.399 s, System: 0.942 s]
 

$ hyperfine -M 5 "dvc repro"
Benchmark 1: dvc repro
  Time (mean ± σ):     634.2 ms ±   4.9 ms    [User: 466.0 ms, System: 162.0 ms]
  Range (min … max):   627.9 ms … 640.9 ms    5 runs
 

$ xvc pipeline new --pipeline-name p100

$ zsh -cl "for f in pipeline-100/dir-0001/* ; do xvc pipeline -p p100 step new -s ${f:r:t} --command 'sha1sum $f' ; xvc pipeline -p p100 step dependency -s ${f:r:t} --file ${f} ; done"

$ hyperfine -r 1 --show-output "xvc pipeline -p p100 run" 
? 1
Benchmark 1: xvc pipeline -n p100 run
error: unexpected argument '-n' found

Usage: xvc pipeline [OPTIONS] <COMMAND>

For more information, try '--help'.
Error: Command terminated with non-zero exit code: 2. Use the '-i'/'--ignore-failure' option if you want to ignore this. Alternatively, use the '--show-output' option to debug what went wrong.

$ hyperfine -M 5 "xvc pipeline -p p100 run"
? 1
Benchmark 1: xvc pipeline -n p100 run
Error: Command terminated with non-zero exit code: 2. Use the '-i'/'--ignore-failure' option if you want to ignore this. Alternatively, use the '--show-output' option to debug what went wrong.

```
