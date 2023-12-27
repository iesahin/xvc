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
  Time (abs ≡):         56.0 ms               [User: 11.3 ms, System: 22.6 ms]
 

$ hyperfine -r 1 'dvc init ; git add .dvc/ .dvcignore ; git commit -m "Init DVC"'
Benchmark 1: dvc init ; git add .dvc/ .dvcignore ; git commit -m "Init DVC"
  Time (abs ≡):        453.7 ms               [User: 205.4 ms, System: 96.1 ms]
 

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
  Time (abs ≡):        606.1 ms               [User: 427.5 ms, System: 152.8 ms]
 

```

Running without changed the dependencies
```console
$ hyperfine -M 5 "dvc repro"
Benchmark 1: dvc repro
  Time (mean ± σ):     442.2 ms ±  19.2 ms    [User: 333.7 ms, System: 102.3 ms]
  Range (min … max):   417.0 ms … 464.7 ms    5 runs
 

```
```console
$ zsh -cl "for f in pipeline-10/dir-0001/* ; do xvc pipeline step new -s ${f:r:t} --command 'sha1sum $f' ; xvc pipeline step dependency -s ${f:r:t} --file ${f} ; done"

$ hyperfine -r 1 "xvc pipeline run"
Benchmark 1: xvc pipeline run
  Time (abs ≡):        301.8 ms               [User: 155.9 ms, System: 303.7 ms]
 

```

```console
$ hyperfine -M 5 "xvc pipeline run"
Benchmark 1: xvc pipeline run
  Time (mean ± σ):     245.4 ms ±   5.0 ms    [User: 137.3 ms, System: 231.6 ms]
  Range (min … max):   240.4 ms … 253.6 ms    5 runs
 

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
s-843    Depends on pipeline-100/dir-0001/file-0001.bin
s-20797  Depends on pipeline-100/dir-0001/file-0002.bin
s-6626   Depends on pipeline-100/dir-0001/file-0003.bin
s-30938  Depends on pipeline-100/dir-0001/file-0004.bin
s-26242  Depends on pipeline-100/dir-0001/file-0005.bin
s-1883   Depends on pipeline-100/dir-0001/file-0006.bin
s-27909  Depends on pipeline-100/dir-0001/file-0007.bin
s-9155   Depends on pipeline-100/dir-0001/file-0008.bin
s-31606  Depends on pipeline-100/dir-0001/file-0009.bin
s-11284  Depends on pipeline-100/dir-0001/file-0010.bin
s-24165  Depends on pipeline-100/dir-0001/file-0011.bin
s-19964  Depends on pipeline-100/dir-0001/file-0012.bin
s-5579   Depends on pipeline-100/dir-0001/file-0013.bin
s-20286  Depends on pipeline-100/dir-0001/file-0014.bin
s-10745  Depends on pipeline-100/dir-0001/file-0015.bin
s-11046  Depends on pipeline-100/dir-0001/file-0016.bin
s-19853  Depends on pipeline-100/dir-0001/file-0017.bin
s-8948   Depends on pipeline-100/dir-0001/file-0018.bin
s-27652  Depends on pipeline-100/dir-0001/file-0019.bin
s-547    Depends on pipeline-100/dir-0001/file-0020.bin
s-2170   Depends on pipeline-100/dir-0001/file-0021.bin
s-592    Depends on pipeline-100/dir-0001/file-0022.bin
s-28760  Depends on pipeline-100/dir-0001/file-0023.bin
s-22381  Depends on pipeline-100/dir-0001/file-0024.bin
s-22540  Depends on pipeline-100/dir-0001/file-0025.bin
s-12712  Depends on pipeline-100/dir-0001/file-0026.bin
s-7330   Depends on pipeline-100/dir-0001/file-0027.bin
s-27934  Depends on pipeline-100/dir-0001/file-0028.bin
s-32060  Depends on pipeline-100/dir-0001/file-0029.bin
s-4171   Depends on pipeline-100/dir-0001/file-0030.bin
s-16230  Depends on pipeline-100/dir-0001/file-0031.bin
s-25307  Depends on pipeline-100/dir-0001/file-0032.bin
s-15288  Depends on pipeline-100/dir-0001/file-0033.bin
s-19474  Depends on pipeline-100/dir-0001/file-0034.bin
s-27427  Depends on pipeline-100/dir-0001/file-0035.bin
s-25775  Depends on pipeline-100/dir-0001/file-0036.bin
s-8219   Depends on pipeline-100/dir-0001/file-0037.bin
s-26912  Depends on pipeline-100/dir-0001/file-0038.bin
s-22151  Depends on pipeline-100/dir-0001/file-0039.bin
s-22736  Depends on pipeline-100/dir-0001/file-0040.bin
s-24235  Depends on pipeline-100/dir-0001/file-0041.bin
s-16259  Depends on pipeline-100/dir-0001/file-0042.bin
s-14398  Depends on pipeline-100/dir-0001/file-0043.bin
s-5695   Depends on pipeline-100/dir-0001/file-0044.bin
s-9975   Depends on pipeline-100/dir-0001/file-0045.bin
s-23044  Depends on pipeline-100/dir-0001/file-0046.bin
s-18203  Depends on pipeline-100/dir-0001/file-0047.bin
s-29608  Depends on pipeline-100/dir-0001/file-0048.bin
s-8348   Depends on pipeline-100/dir-0001/file-0049.bin
s-29075  Depends on pipeline-100/dir-0001/file-0050.bin
s-30021  Depends on pipeline-100/dir-0001/file-0051.bin
s-10098  Depends on pipeline-100/dir-0001/file-0052.bin
s-13091  Depends on pipeline-100/dir-0001/file-0053.bin
s-29020  Depends on pipeline-100/dir-0001/file-0054.bin
s-2960   Depends on pipeline-100/dir-0001/file-0055.bin
s-16839  Depends on pipeline-100/dir-0001/file-0056.bin
s-492    Depends on pipeline-100/dir-0001/file-0057.bin
s-16379  Depends on pipeline-100/dir-0001/file-0058.bin
s-3876   Depends on pipeline-100/dir-0001/file-0059.bin
s-3854   Depends on pipeline-100/dir-0001/file-0060.bin
s-26927  Depends on pipeline-100/dir-0001/file-0061.bin
s-9169   Depends on pipeline-100/dir-0001/file-0062.bin
s-2407   Depends on pipeline-100/dir-0001/file-0063.bin
s-21917  Depends on pipeline-100/dir-0001/file-0064.bin
s-14842  Depends on pipeline-100/dir-0001/file-0065.bin
s-29742  Depends on pipeline-100/dir-0001/file-0066.bin
s-1203   Depends on pipeline-100/dir-0001/file-0067.bin
s-5241   Depends on pipeline-100/dir-0001/file-0068.bin
s-7308   Depends on pipeline-100/dir-0001/file-0069.bin
s-16016  Depends on pipeline-100/dir-0001/file-0070.bin
s-28584  Depends on pipeline-100/dir-0001/file-0071.bin
s-2201   Depends on pipeline-100/dir-0001/file-0072.bin
s-4437   Depends on pipeline-100/dir-0001/file-0073.bin
s-30530  Depends on pipeline-100/dir-0001/file-0074.bin
s-10687  Depends on pipeline-100/dir-0001/file-0075.bin
s-26198  Depends on pipeline-100/dir-0001/file-0076.bin
s-17179  Depends on pipeline-100/dir-0001/file-0077.bin
s-21972  Depends on pipeline-100/dir-0001/file-0078.bin
s-26033  Depends on pipeline-100/dir-0001/file-0079.bin
s-28125  Depends on pipeline-100/dir-0001/file-0080.bin
s-377    Depends on pipeline-100/dir-0001/file-0081.bin
s-15125  Depends on pipeline-100/dir-0001/file-0082.bin
s-24940  Depends on pipeline-100/dir-0001/file-0083.bin
s-10642  Depends on pipeline-100/dir-0001/file-0084.bin
s-28914  Depends on pipeline-100/dir-0001/file-0085.bin
s-11870  Depends on pipeline-100/dir-0001/file-0086.bin
s-22928  Depends on pipeline-100/dir-0001/file-0087.bin
s-7539   Depends on pipeline-100/dir-0001/file-0088.bin
s-994    Depends on pipeline-100/dir-0001/file-0089.bin
s-1175   Depends on pipeline-100/dir-0001/file-0090.bin
s-22893  Depends on pipeline-100/dir-0001/file-0091.bin
s-5583   Depends on pipeline-100/dir-0001/file-0092.bin
s-21790  Depends on pipeline-100/dir-0001/file-0093.bin
s-24663  Depends on pipeline-100/dir-0001/file-0094.bin
s-28835  Depends on pipeline-100/dir-0001/file-0095.bin
s-26067  Depends on pipeline-100/dir-0001/file-0096.bin
s-16036  Depends on pipeline-100/dir-0001/file-0097.bin
s-6020   Depends on pipeline-100/dir-0001/file-0098.bin
s-32666  Depends on pipeline-100/dir-0001/file-0099.bin
s-25255  Depends on pipeline-100/dir-0001/file-0100.bin

$ hyperfine -r 1 "dvc repro"
Benchmark 1: dvc repro
  Time (abs ≡):         9.498 s               [User: 8.433 s, System: 0.919 s]
 

$ hyperfine -M 5 "dvc repro"
Benchmark 1: dvc repro
  Time (mean ± σ):     630.7 ms ±   9.4 ms    [User: 464.1 ms, System: 160.0 ms]
  Range (min … max):   619.0 ms … 641.4 ms    5 runs
 

$ xvc pipeline new --pipeline-name p100

$ zsh -cl "for f in pipeline-100/dir-0001/* ; do xvc pipeline -n p100 step new -s ${f:r:t} --command 'sha1sum $f' ; xvc pipeline -n p100 step dependency -s ${f:r:t} --file ${f} ; done"

$ hyperfine -r 1 "xvc pipeline -n p100 run"
Benchmark 1: rm -f $TEMPDIR/xvc.log ; xvc --debug pipeline -n p100 run
  Time (abs ≡):        11.400 s               [User: 43.921 s, System: 18.029 s]
 

$ hyperfine -M 5 "xvc pipeline -n p100 run"
Benchmark 1: xvc pipeline -n p100 run
  Time (mean ± σ):     316.0 ms ±   6.0 ms    [User: 181.4 ms, System: 400.0 ms]
  Range (min … max):   309.4 ms … 323.6 ms    5 runs
 

```
