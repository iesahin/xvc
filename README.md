# xvc

[![codecov](https://codecov.io/gh/iesahin/xvc/branch/main/graph/badge.svg?token=xa3ru5KhRq)](https://codecov.io/gh/iesahin/xvc)
[![build](https://img.shields.io/github/actions/workflow/status/iesahin/xvc/rust.yml?branch=main)](https://github.com/iesahin/xvc/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/xvc)](https://crates.io/crates/xvc)
[![docs.rs](https://img.shields.io/docsrs/xvc)](https://docs.rs/xvc/)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

A fast and robust MLOps tool to manage data and pipelines

## ⌛ When to use xvc?

- When you have a photo, audio, media, or document collection to backup/version with Git, but don't want to copy that huge data to all Git clones.
- When you manage a large number of _unstructured_ data, like images, documents, and audio files.
- When you want to version data files, and want to track versions across datasets.
- When you want to store this data in local, SSH-accessible, or S3-compatible cloud storage.
- When you create data pipelines on top of this data and want to run these pipelines when the data, code, or other dependencies change.
- When you want to track which subset of the data you're working with, and how it changes by your operations.
- When you have binary artifacts that you use as dependencies and would like to have a `make` alternative that considers _content changes_ rather than timestamps.

## ✳️ What is xvc for?

- (for x = files) Track large files on Git, store them in the cloud, create view-only subsets, retrieve them only when necessary.
- (for x = pipelines) Define and run data -> model pipelines whose dependencies may be files, hyperparameters, regex searches, arbitrary URLs, and more.
- (for x = data) Annotate data and run queries and retrieve subsets of it. ([TODO](https://github.com/iesahin/xvc/discussions/208))
- (for x = experiments) Run isolated experiments, share them and store them in Git when necessary ([TODO](https://github.com/iesahin/xvc/discussions/207))
- (for x = models) Associate models with datasets, metadata and features, then track, store, and deploy them ([TODO](https://github.com/iesahin/xvc/discussions/211))

## 🔽 Installation

You can get the binary files for Linux, macOS, and Windows from [releases](https://github.com/iesahin/xvc/releases/latest) page. Extract and copy the file to your `$PATH`.

Alternatively, if you have Rust [installed], you can build xvc:

```shell
$ cargo install xvc
```

[installed]: https://www.rust-lang.org/tools/install

## 🏃🏾 Quicktart

Xvc tracks your files and directories on top of Git. To start run the following command in the repository.

```shell
$ git init # if you're not already in a Git repository
$ xvc init
```

It initializes the metafiles in `.xvc/` directory and adds `.xvcignore` file for paths you may want to hide from Xvc.

Add your data files and directories for tracking.

```shell
$ xvc file track my-data/ --cache-type symlink
```

The command calculates data content hashes (with BLAKE-3, by default) and records them.
It commits these changes to Git.
It also copies these files to content-addressed directories under `.xvc/b3` and creates read-only symbolic links to them.

You can specify different types of [cache-types] specific to files and directories, for your use case.
If you need to track model files that change frequently, you can set `--cache-type copy` (the default).

```shell
$ xvc file track my-models/ --cache-type copy
```

When you want to share them, configure a storage to share the files you added.

```shell
$ xvc storage new s3 --name my-remote --region us-east-1 --bucket-name my-xvc-remote
```

You can send the files to this storage.

```shell
$ xvc file send --to my-remote
```

When you (or someone else) want to access these files later, you can clone the Git repository and get the files from storage.

```console
$ git clone https://example.com/my-machine-learning-project
$ cd my-machine-learning-project
$ xvc file bring my-data/ --from my-remote
```

You don't have to reconfigure the storage after cloning, but you need to have valid credentials as environment variables
to access the storage.
Xvc doesn't store any credentials.

If you have commands that depend on data or code elements, you can configure a pipeline.

Create a step for each command.

```shell
$ xvc pipeline step new --step-name preprocess --command 'python3 preprocess.py'
$ xvc pipeline step new --step-name train --command 'python3 train.py'
$ xvc pipeline step new --step-name test --command 'python3 test.py'
```

Then, configure dependencies between these steps.

```console
$ xvc pipeline step dependency --step-name preprocess --glob 'my-data/*.jpg' \
                                                      --file preprocess.py \
                                                      --regex 'names.txt:/^Name:' \
                                                      --lines a-long-file.csv::-1000
$ xvc pipeline step dependency --step-name train  --step preprocess
$ xvc pipeline step dependency --step-name test   --file test-data.npz \
                                                  --file my-models/model.h5
$ xvc pipeline step output --step-name preprocess --output-file test-data.npz
$ xvc pipeline step output --step-name train --output-file my-models/model.h5
```


The above commands define three steps in `default` pipeline. You can have multiple pipelines if you need.

The first is `preprocess` that depends on 'jpg' files in `my-data/` directory, lines that start with `Name:` in `names.txt`; and the first 1000 lines in `a-long-file.csv`. It also depends on the script itself, so when you make changes to the script itself, it invalidates the step.
The second step is called `train`. It depends on `preprocess` step directly, anything that make `preprocess` to rerun, makes `train` to run as well.
The `test` step depends on `train` and `preprocess` via their outputs. It's run when these outputs (`test-data.npz` and `model.h5`) are changed.

You can get the pipeline in Graphviz DOT format to convert to an image.

```console
$ xvc pipeline dag
digraph {
    0 [ label = "step: train (by_dependencies, python3 train.py)" ]
    1 [ label = "step: preprocess (by_dependencies, python3 preprocess.py)" ]
    2 [ label = "step: test (by_dependencies, python3 test.py)" ]
    3 [ label = "file: my-models/model.h5" ]
    4 [ label = "file: test-data.npz" ]
    0 -> 1 [ label = "" ]
    2 -> 3 [ label = "" ]
    2 -> 4 [ label = "" ]
}
```

You can also export and import the pipeline to JSON to edit in your editor.

```console
$ xvc pipeline export > my-pipeline.json
$ nvim my-pipeline.json
$ xvc pipeline import --file my-pipeline.json --overwrite
```

You can run the pipeline with.

```shell
$ xvc pipeline run
```

If the steps you defined doesn't depend to each other, they are run in parallel.

You can define fairly complex dependencies with globs, files, directories, regular expression searches in files, lines in files, other steps and pipelines with `xvc pipeline step dependency` commands.
More dependency types like database queries, content from URLs, S3 (and compatible) buckets, REST and GraphQL results are in the backlog.
Please create an issue or discussion for any other kinds of dependencies that you'd like to be included.

Please check [xvc.netlify.app](https://docs.xvc.dev) for documentation.

## 🤟 Big Thanks

xvc stands on the following (giant) crates:

- [trycmd] is used to run all example commands in the [reference and how-to documentation](https://docs.xvc.dev) at
  every PR. It makes sure that the documentation is always up-to-date and shown commands work as described. We start
  development by writing documentation and implementing them thanks to [trycmd].

- [serde] allows all data structures to be stored in text files. Special thanks from [`xvc-ecs`] for serializing components in an ECS with a single line of code.

- Xvc processes files in parallel with pipelines and parallel iterators thanks to [crossbeam] and [rayon].

- Thanks to [strum], Xvc uses enums extensively and converts almost everything to typed values from strings.

- Xvc has a deep CLI that has subcommands of subcommands (e.g. `xvc storage new s3`), and all these work with minimum bugs thanks to [clap].

- Xvc uses [rust-s3] to connect to S3 and compatible storage services. It employs excellent [tokio] for fast async Rust. These cloud storage features can be turned off thanks to Rust conditional compilation.

- Without implementations of [BLAKE3], BLAKE2, SHA-2 and SHA-3 from Rust [crypto] crate, Xvc couldn't detect file changes so fast.

- Many thanks to small and well built crates, [reflink], [relative-path], [path-absolutize], [glob] for file system and glob handling.

- Thanks to [sad_machine] for providing a State Machine implementation that I used in `xvc pipeline run`. A DAG composed of State Machines made running pipeline steps in parallel with a clean separation of process states.

- Thanks to [thiserror] and [anyhow] for making error handling a breeze. These two crates make me feel I'm doing something good for the humanity when handling errors.

- Xvc is split into many crates and owes this organization to [cargo workspaces].

[crossbeam]: https://docs.rs/crossbeam/
[cargo workspaces]: https://crates.io/crates/cargo-workspaces
[rayon]: https://docs.rs/rayon/
[strum]: https://docs.rs/strum/
[clap]: https://docs.rs/clap/
[serde]: https://serde.rs
[blake3]: https://docs.rs/blake3/
[crypto]: https://docs.rs/rust-crypto/
[reflink]: https://docs.rs/reflink/
[relative-path]: https://docs.rs/relative-path/
[path-absolutize]: https://docs.rs/path-absolutize/
[glob]: https://docs.rs/glob/
[wax]: https://docs.rs/wax/
[trycmd]: https://docs.rs/trycmd/
[sad_machine]: https://docs.rs/sad_machine/
[thiserror]: https://docs.rs/thiserror/
[anyhow]: https://docs.rs/anyhow/
[rust-s3]: https://docs.rs/rust-s3/
[`xvc-ecs`]: https://docs.rs/xvc-ecs/
[tokio]: https://tokio.rs

And, biggest thanks to Rust designers, developers and contributors. Although I can't see myself expert to appreciate it all, it's a fabulous language and environment to work with.

## 🚁 Support

- You can use [Discussions](https://github.com/iesahin/xvc/discussions) to ask questions. I'll answer as much as possible. Thank you.
- I don't follow any other sites regularly. You can also reach me at [emre@xvc.dev](mailto:emre@xvc.dev)

## 👐 Contributing

- Star this repo. I feel very happy for five minutes for every star and send my best wishes to you. That's a certain win to spend your two seconds for me. Thanks.
- Use xvc. Tell me how it works for you, read the [documentation](https://docs.xvc.dev), [report bugs](https://github.com/iesahin/xvc/issues), [discuss features](https://github.com/iesahin/xvc/discussions).
- Note that, I don't accept large code PRs. Please open an issue to discuss your idea and write/modify a
  reference page before sending a PR. I'm happy to discuss and help you to implement your idea.

## 📜 License

Xvc is licensed under the [Apache 2.0 License](https://github.com/iesahin/xvc/blob/main/LICENSE).

## 🌦️ Future and Maintenance

This is mostly a one-man project and users may consider the [bus factor](https://en.wikipedia.org/wiki/Bus_factor) before spending time on it.

I'm using Xvc daily and I'm happy with it. I'll maintain it as long as I use it. I'm applying my
technical/architectural ideas to see their effectiveness and I have more ideas to implement. I don't expect to be bored from this soon.

I'm in a phase of my life where material success doesn't entice me. I have a daily routine that I love and it won't change much if I earn a billion dollars. I don't want to convert Xvc to a business and have
more than one goal with this project. In my opinion, trying to monetize OSS prematurely deteriorates it more than other factors these days.

Nevertheless, Xvc is like a _running CV_ for me, basically signaling _I can do this and I can do similar software for you._ This is another motivation for me to keep it alive.

## ⚠️ Disclaimer

This software is fresh and ambitious. Although I use it and test it close to real world conditions, it didn't go under test of time. **Xvc can eat your files and spit them to eternal void!**
