# How to create a data pipeline with Xvc

A data pipeline starts from data and ends with models. Between there is various data transformations and model training. We try to make all pieces reproducible and Xvc helps with this goal. 

In this document, we'll create the following pipeline for a digit recognition system. Our purpose is to show how Xvc helps in versioning data, so this document doesn't try to achieve a high classification performance. 

```mermaid
graph LR
A[Data Gathering] --> B[Splitting Test and Train Sets]
B --> C[Preprocessing Images into Numpy Arrays]
C --> D[Training Model]
D --> E[Sharing Data and Models]
```


```admonish info
This document can be more verbose than usual, because all commands in this document are run on a clean directory during tests to check outputs. Some of the idiosyncrasies, e.g., running certain commands with `zsh -c` are due to this reason. Some of the output that changes in 
```
```
```

Although you can do without, most of the times Xvc runs in a Git repository. This allows to version control both the data and the code together. 
```console
$ git init
Initialized empty Git repository in [CWD]/.git/

$ xvc init
```

In this HOWTO, we use Chinese MNIST dataset to create an image classification pipeline. We already downloaded it [from kaggle](https://www.kaggle.com/datasets/gpreda/chinese-mnist/data). 

```console
$ ls -l
total 21112
-rw-r--r--  1 iex  staff  10792680 Nov 17 19:46 chinese_mnist.zip
-rw-r--r--  1 iex  staff      1124 Nov 28 14:27 image_to_numpy_array.py
-rw-r--r--  1 iex  staff        14 Nov 28 14:36 requirements.txt
-rw-r--r--  1 iex  staff      4266 Nov 30 22:14 train.py

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
total 32
lrwxr-xr-x  1 iex  staff   195 Dec  1 10:55 chinese_mnist.zip -> [CWD]/.xvc/b3/b24/2c9/422f91b804ea3008bc0bc025e97bf50c1d902ae7a0f13588b84f59023d/0.zip
-rw-r--r--  1 iex  staff  1124 Nov 28 14:27 image_to_numpy_array.py
-rw-r--r--  1 iex  staff    14 Nov 28 14:36 requirements.txt
-rw-r--r--  1 iex  staff  4266 Nov 30 22:14 train.py

```

The long directory name is the BLAKE-3 hash of the data file.

As we'll work with the file contents, let's unzip the data file.

```console
$ unzip -q chinese_mnist.zip

$ ls -l
total 32
lrwxr-xr-x  1 iex  staff   195 Dec  1 10:55 chinese_mnist.zip -> [CWD]/.xvc/b3/b24/2c9/422f91b804ea3008bc0bc025e97bf50c1d902ae7a0f13588b84f59023d/0.zip
drwxr-xr-x  4 iex  staff   128 Nov 17 19:45 data
-rw-r--r--  1 iex  staff  1124 Nov 28 14:27 image_to_numpy_array.py
-rw-r--r--  1 iex  staff    14 Nov 28 14:36 requirements.txt
-rw-r--r--  1 iex  staff  4266 Nov 30 22:14 train.py

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
SS         [..] 3a714d65          data/data/input_9_9_9.jpg
SS         [..] 9ffccc4d          data/data/input_9_9_8.jpg
SS         [..] 5d6312a4          data/data/input_9_9_7.jpg
SS         [..] 7a0ddb0e          data/data/input_9_9_6.jpg
SS         [..] 2047d7f3          data/data/input_9_9_5.jpg
SS         [..] 10fcf309          data/data/input_9_9_4.jpg
SS         [..] 0bdcd918          data/data/input_9_9_3.jpg
SS         [..] aebcbc03          data/data/input_9_9_2.jpg
SS         [..] 38abd173          data/data/input_9_9_15.jpg
SS         [..] 7c6a9003          data/data/input_9_9_14.jpg
SS         [..] a9f04ad9          data/data/input_9_9_13.jpg
SS         [..] 2d372f95          data/data/input_9_9_12.jpg
SS         [..] 8fe799b4          data/data/input_9_9_11.jpg
SS         [..] ee35e5d5          data/data/input_9_9_10.jpg
SS         [..] 7576894f          data/data/input_9_9_1.jpg
Total #: 15 Workspace Size:        2925 Cached Size:        8710


```

`xvc file list` command shows the tracking status. Initial two characters shows
the tracking status, `SS` means the file is tracked as symlink and is available
in the workspace as a symlink. The next column shows the file size, then the
last modified date, then the BLAKE-3 hash of the file, and finally the file
name. The empty column contains the actual hash of the file if the file is
available in the workspace. Here it's empty because the workspace file is a
link to the file in cache.

The summary line shows the total size of the files and the size they occupy in
the workspace.

## Splitting Train, Validation, and Test Sets

The first step of the pipeline is to create subsets of the data. 

The data set contains 15 classes. It has 10 samples for each of these classes
from 100 different people. As we'll train a Chinese digit recognizer, we'll
first divide volunteers 1-60 for training, 61-80 for validation, and 81-100 for
testing. This will ensure that the model is not trained with the same person's
handwriting.


```console
$ xvc file copy --name-only data/data/input_?_* data/train/
$ xvc file copy --name-only data/data/input_[12345]?_* data/train/
$ xvc file copy --name-only data/data/input_100_* data/train/
$ xvc file copy --name-only data/data/input_[67]?_* data/validate/
$ xvc file copy --name-only data/data/input_[89]?_* data/test/

$ tree -d data/
data/
├── data
├── test
├── train
└── validate

5 directories

```

If you look at the contents of these directories, you'll see that they are
symbolic links to the same files we started to track. 

Let's check the number of images in each set. 

```console
$ zsh -c 'ls -1 data/train/*.jpg | wc -l'
    9000

$ zsh -c 'ls -1 data/validate/*.jpg | wc -l'
    3000

$ zsh -c 'ls -1 data/test/*.jpg | wc -l'
    3000

```

The first step in the pipeline will be rechecking (checking out) these subsets.

```console
$ xvc pipeline step new -s recheck-data --command 'xvc file recheck data/train/ data/validate/ data/test/'
```

[`xvc file recheck`](/ref/xvc-file-recheck.md) is used in to instate files from Xvc cache.
Let's test the pipeline by first deleting the files we manually created.
```console
$ rm -rf data/train data/validate data/test
```

We run the steps we created.

```console
$ xvc pipeline run 
[DONE] recheck-data (xvc file recheck data/train/ data/validate/ data/test/)

```
If we check the contents of the directories, we'll see that they are back.

```console
$ zsh -c 'ls -1 data/train/*.jpg | wc -l'
    9000

```

## Preprocessing Images into Numpy Arrays

```mermaid
graph LR
A[Data Gathering ✅]  --> B[Splitting Test and Train Sets ✅]
B --> C[Preprocessing Images into Numpy Arrays]
C --> D[Training Model]
D --> E[Sharing Data and Models]
```

The Python script to train a model runs with Numpy arrays. So we'll convert each of these directories with images into two numpy arrays. 
One of the arrays will keep $n$ 64x64 images and the other will keep $n$ labels for these images.

```console
$ xvc pipeline step new --step-name create-train-array --command '.venv/bin/python3 image_to_numpy_array.py --dir data/train/'
$ xvc pipeline step new --step-name create-test-array --command '.venv/bin/python3 image_to_numpy_array.py --dir data/test/'
$ xvc pipeline step new --step-name create-validate-array --command '.venv/bin/python3 image_to_numpy_array.py --dir data/validate/'
```

These commands will run when the image files in those directories will change. Xvc can keep track of file groups and invalidate a step when the _content_ of any of these files change. Moreover, it's possible to track which files have changed if there are too many files. We don't need this feature of tracking individual items in _globs_, so we'll use a _glob_ dependency. 

```console
$ xvc pipeline step dependency --step-name create-train-array --glob 'data/train/*.jpg'
$ xvc pipeline step dependency --step-name create-test-array --glob 'data/test/*.jpg'
$ xvc pipeline step dependency --step-name create-validate-array --glob 'data/validate/*.jpg'
```

Now we have three more steps that depend on changed files. The script depends on OpenCV to read images. Python best practices recommend to create a separate virtual environment for each project. We'll also make sure that the venv is created and the requirements are installed before running the script.

Create a command to initialize the virtual environment. It will run if there is no `.venv/bin/activate` file. 

```console
$ xvc pipeline step new --step-name init-venv --command 'python3 -m venv .venv'
$ xvc pipeline step dependency --step-name init-venv --generic 'echo "$(hostname)/$(pwd)"'
```

We used `--generic` dependency that runs a command and checks its output to see whether the step requires to be run again. We only want to run `init-env` once per deployment, so checking output of `hostname` and `pwd` is better than existence of a file. File dependencies must be available before running the pipeline to record their metadata. There is no such restriction for generic dependencies.

Then, another step that depends on `init-venv` and `requirements.txt` will install the dependencies. 

```console
$ xvc pipeline step new --step-name install-requirements --command '.venv/bin/python3 -m pip install -r requirements.txt'
$ xvc pipeline step dependency --step-name install-requirements --step init-venv
$ xvc pipeline step dependency --step-name install-requirements --file requirements.txt
```
Note that, unlike other tools, you can specify direct dependencies between steps in Xvc. When a pipeline step must wait another step to finish successfully, a dependency between these two can be defined. 

The above `create-*-array` steps will depend on to `install-requirements` to ensure that requirements are installed when the scripts are run. 

```console
$ xvc pipeline step dependency --step-name create-train-array --step install-requirements

$ xvc pipeline step dependency --step-name create-validate-array --step install-requirements

$ xvc pipeline step dependency --step-name create-test-array --step install-requirements

```

Now, as the pipeline grows, it may be nice to see the graph what we have done so far. 

```console
$ xvc pipeline dag --format mermaid
flowchart TD
    n0["recheck-data"]
    n1["create-train-array"]
    n2["data/train/*.jpg"] --> n1
    n3["install-requirements"] --> n1
    n4["create-test-array"]
    n5["data/test/*.jpg"] --> n4
    n3["install-requirements"] --> n4
    n6["create-validate-array"]
    n7["data/validate/*.jpg"] --> n6
    n3["install-requirements"] --> n6
    n8["init-venv"]
    n9["echo "$(hostname)/$(pwd)""] --> n8
    n3["install-requirements"]
    n8["init-venv"] --> n3
    n10["requirements.txt"] --> n3


```
```mermaid
flowchart TD
    n0["recheck-data"]
    n1["create-train-array"]
    n2["data/train/*.jpg"] --> n1
    n3["install-requirements"] --> n1
    n4["create-test-array"]
    n5["data/test/*.jpg"] --> n4
    n3["install-requirements"] --> n4
    n6["create-validate-array"]
    n7["data/validate/*.jpg"] --> n6
    n3["install-requirements"] --> n6
    n8["init-venv"]
    n9[".venv/bin/activate"] --> n8
    n3["install-requirements"]
    n8["init-venv"] --> n3
    n10["requirements.txt"] --> n3
```

`dag` command can also produce GraphViz DOT output. For larger graphs, it may be more suitable. We'll use DOT to create images in later sections. 

Let's run the pipeline at this point to test.

```console
$ xvc -vv pipeline run
[INFO] Found explicit dependency: XvcStep { name: "create-test-array" } -> Step(StepDep { name: "install-requirements" })
[INFO] Found explicit dependency: XvcStep { name: "create-validate-array" } -> Step(StepDep { name: "install-requirements" })
[INFO] Found explicit dependency: XvcStep { name: "install-requirements" } -> Step(StepDep { name: "init-venv" })
[INFO] Found explicit dependency: XvcStep { name: "create-train-array" } -> Step(StepDep { name: "install-requirements" })
[INFO][pipeline/src/pipeline/mod.rs::343] Pipeline Graph:
digraph {
    0 [ label = "(30009, 3133346587219904106)" ]
    1 [ label = "(30011, 3978600373243050192)" ]
    2 [ label = "(30012, 7097585986794152488)" ]
    3 [ label = "(30018, 7014435414431453876)" ]
    4 [ label = "(30010, 15781307687659832701)" ]
    5 [ label = "(30016, 17174800164399411076)" ]
    1 -> 3 [ label = "Step" ]
    2 -> 3 [ label = "Step" ]
    3 -> 5 [ label = "Step" ]
    4 -> 3 [ label = "Step" ]
}


[INFO] Waiting for dependency steps for step create-train-array
[INFO] Waiting for dependency steps for step create-test-array
[INFO] Waiting for dependency steps for step create-validate-array
[INFO] Waiting for dependency steps for step install-requirements
[INFO] No dependency steps for step init-venv
[INFO] No dependency steps for step recheck-data
[INFO] [recheck-data] Dependencies has changed
[INFO] [init-venv] Dependencies has changed
[DONE] recheck-data (xvc file recheck data/train/ data/validate/ data/test/)
[DONE] init-venv (python3 -m venv .venv)
[INFO] Dependency steps completed successfully for step install-requirements
[INFO] [install-requirements] Dependencies has changed
[OUT] [install-requirements] Collecting opencv-python (from -r requirements.txt (line 1))
  Using cached opencv_python-4.8.1.78-cp37-abi3-macosx_11_0_arm64.whl.metadata (19 kB)
Collecting numpy>=1.21.2 (from opencv-python->-r requirements.txt (line 1))
  Using cached numpy-1.26.2-cp311-cp311-macosx_11_0_arm64.whl.metadata (115 kB)
Using cached opencv_python-4.8.1.78-cp37-abi3-macosx_11_0_arm64.whl (33.1 MB)
Using cached numpy-1.26.2-cp311-cp311-macosx_11_0_arm64.whl (14.0 MB)
Installing collected packages: numpy, opencv-python
Successfully installed numpy-1.26.2 opencv-python-4.8.1.78
 
[DONE] install-requirements (.venv/bin/python3 -m pip install -r requirements.txt)
[INFO] Dependency steps completed successfully for step create-train-array
[INFO] Dependency steps completed successfully for step create-test-array
[INFO] Dependency steps completed successfully for step create-validate-array
[INFO] [create-validate-array] Dependencies has changed
[INFO] [create-test-array] Dependencies has changed
[INFO] [create-train-array] Dependencies has changed
[DONE] create-validate-array (.venv/bin/python3 image_to_numpy_array.py --dir data/validate/)
[DONE] create-test-array (.venv/bin/python3 image_to_numpy_array.py --dir data/test/)
[DONE] create-train-array (.venv/bin/python3 image_to_numpy_array.py --dir data/train/)
[WARN][walker/src/error.rs::91] Crossbeam Send Error for Type: "Some(/n    Create {/n        path: /"[CWD]/.xvc/store/xvc-dependency-store/1701417399936438.json/",/n        metadata: Metadata {/n            file_type: FileType(/n                FileType {/n                    mode: 33188,/n                },/n            ),/n            is_dir: false,/n            is_file: true,/n            permissions: Permissions(/n                FilePermissions {/n                    mode: 33188,/n                },/n            ),/n            modified: Ok(/n                SystemTime {/n                    tv_sec: 1701417399,/n                    tv_nsec: 936725061,/n                },/n            ),/n            accessed: Ok(/n                SystemTime {/n                    tv_sec: 1701417399,/n                    tv_nsec: 936642854,/n                },/n            ),/n            created: Ok(/n                SystemTime {/n                    tv_sec: 1701417399,/n                    tv_nsec: 936642854,/n                },/n            ),/n            ../n        },/n    },/n)" "sending on a disconnected channel"
[WARN][walker/src/error.rs::91] Crossbeam Send Error for Type: "Some(/n    Create {/n        path: /"[CWD]/.xvc/store/xvc-dependency-store/1701417399936438.json/",/n        metadata: Metadata {/n            file_type: FileType(/n                FileType {/n                    mode: 33188,/n                },/n            ),/n            is_dir: false,/n            is_file: true,/n            permissions: Permissions(/n                FilePermissions {/n                    mode: 33188,/n                },/n            ),/n            modified: Ok(/n                SystemTime {/n                    tv_sec: 1701417399,/n                    tv_nsec: 936725061,/n                },/n            ),/n            accessed: Ok(/n                SystemTime {/n                    tv_sec: 1701417399,/n                    tv_nsec: 936642854,/n                },/n            ),/n            created: Ok(/n                SystemTime {/n                    tv_sec: 1701417399,/n                    tv_nsec: 936642854,/n                },/n            ),/n            ../n        },/n    },/n)" "sending on a disconnected channel"

```

Now, when we take a look at the data directories, we find `images.npy` and `classes.npy` files.

```console
$ zsh -cl 'ls -l data/train/*.npy'
-rw-r--r--  1 iex  staff      72128 Dec  1 10:56 data/train/classes.npy
-rw-r--r--  1 iex  staff  110592128 Dec  1 10:56 data/train/images.npy

$ zsh -cl 'ls -l data/test/*.npy'
-rw-r--r--  1 iex  staff     24128 Dec  1 10:56 data/test/classes.npy
-rw-r--r--  1 iex  staff  36864128 Dec  1 10:56 data/test/images.npy

$ zsh -cl 'ls -l data/validate/*.npy'
-rw-r--r--  1 iex  staff     24128 Dec  1 10:56 data/validate/classes.npy
-rw-r--r--  1 iex  staff  36864128 Dec  1 10:56 data/validate/images.npy

```

## Train a model

Now we have built the NumPy arrays, we can train a model. We'll use a simple convolutional neural network as a showcase. This is by no means a state-of-art solution, so the results will be less than perfect.

```mermaid
graph LR
A[Data Gathering ✅]  --> B[Splitting Test and Train Sets ✅]
B --> C[Preprocessing Images into Numpy Arrays ✅]
C --> D[Training Model]
D --> E[Sharing Data and Models]
```

The script receives training, validation and testing directories, loads the data from Numpy arrays we just produced, loads hyperparameters from a file called `params.yaml`, trains the model, tests it and writes the results and model to a file. It's a very involved piece produced with the assistance of GPT-4. 

We first define the step to run the command:

```console
$ xvc pipeline step new --step-name train-model --command '.venv/bin/python3 train.py  --train_dir data/train/ --validation_dir data/validate --test_dir data/test'

```

The step will depend to array generation steps by depending on the files they produce. In order to define a dependency between `train-model` and `create-train-array` step, we must tell that `create-array-dependency` outputs a file called `images.npy`. We can do this by using `--file` option of `step output` command. 
```console
$ xvc pipeline step output --step-name create-train-array --output-file data/train/images.npy

$ xvc pipeline step output --step-name create-train-array --output-file data/train/classes.npy

$ xvc pipeline step dependency --step-name train-model --file data/train/images.npy
$ xvc pipeline step dependency --step-name train-model --file data/train/classes.npy
```

Note that this operation is different from creating a direct dependency between steps. There may be multiple steps creating the same outputs and there may be multiple steps depending on the same files. Preferring direct (`--step`) dependencies and indirect (`--file`) dependencies is a matter of taste and use. 

We'll create these dependencies for other files as well. 

```console
$ xvc pipeline step output --step-name create-test-array --output-file data/test/images.npy

$ xvc pipeline step output --step-name create-test-array --output-file data/test/classes.npy

$ xvc pipeline step dependency --step-name train-model --file data/test/images.npy

$ xvc pipeline step dependency --step-name train-model --file data/test/classes.npy

$ xvc pipeline step output --step-name create-validate-array --output-file data/validate/images.npy

$ xvc pipeline step output --step-name create-validate-array --output-file data/validate/classes.npy

$ xvc pipeline step dependency --step-name train-model --file data/validate/images.npy

$ xvc pipeline step dependency --step-name train-model --file data/validate/classes.npy

```

Before running the pipeline, let's see the pipeline DAG once more. This time in DOT format. 

```console
$ xvc pipeline dag 
digraph pipeline{n0[shape=box;label="recheck-data";];n1[shape=box;label="create-train-array";];n2[shape=folder;label="data/train/*.jpg";];n2->n1;n3[shape=box;label="install-requirements";];n3->n1;n4[shape=note;color=black;label="data/train/images.npy";];n1->n4;n5[shape=note;color=black;label="data/train/classes.npy";];n1->n5;n6[shape=box;label="create-test-array";];n7[shape=folder;label="data/test/*.jpg";];n7->n6;n3[shape=box;label="install-requirements";];n3->n6;n8[shape=note;color=black;label="data/test/images.npy";];n6->n8;n9[shape=note;color=black;label="data/test/classes.npy";];n6->n9;n10[shape=box;label="create-validate-array";];n11[shape=folder;label="data/validate/*.jpg";];n11->n10;n3[shape=box;label="install-requirements";];n3->n10;n12[shape=note;color=black;label="data/validate/images.npy";];n10->n12;n13[shape=note;color=black;label="data/validate/classes.npy";];n10->n13;n14[shape=box;label="init-venv";];n15[shape=trapezium;label="echo /"$(hostname)/$(pwd)/"";];n15->n14;n3[shape=box;label="install-requirements";];n14[shape=box;label="init-venv";];n14->n3;n16[shape=note;label="requirements.txt";];n16->n3;n17[shape=box;label="train-model";];n4[shape=note;label="data/train/images.npy";];n4->n17;n5[shape=note;label="data/train/classes.npy";];n5->n17;n8[shape=note;label="data/test/images.npy";];n8->n17;n9[shape=note;label="data/test/classes.npy";];n9->n17;n12[shape=note;label="data/validate/images.npy";];n12->n17;n13[shape=note;label="data/validate/classes.npy";];n13->n17;}

```

It's not very readable but you can feed the output to `dot` command to create a PNG file. 
```console
$ zsh -cl 'xvc pipeline dag | dot -Tsvg > pipeline.svg'

```


