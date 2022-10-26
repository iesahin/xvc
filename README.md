# xvc

[![codecov](https://codecov.io/gh/iesahin/xvc/branch/master/graph/badge.svg?token=yrcNOA4RTy)](https://codecov.io/gh/iesahin/xvc)
[![build](https://img.shields.io/github/workflow/status/iesahin/xvc/Rust-CI)](https://github.com/iesahin/xvc/actions/workflows/rust.yml)]
[![crates.io](https://img.shields.io/crates/v/xvc)](https://crates.io/crates/xvc)
[![docs.rs](https://img.shields.io/docsrs/xvc)](https://docs.rs/xvc/)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

A Fast and Robust MLOps Swiss-Army Knife in Rust

## ⌛ When to use xvc?

- Machine Learning Engineers: When you manage large quantities of _unstructured_ data, like images, documents, audio files. When you create data pipelines on top of this data and want to run these pipelines when the data, code or other dependencies change.
- Data Engineers: When you want to version data files, and want to track versions across datasets. When you have to provide this data in multiple remote locations, like S3 or local files.
- Data Scientists: When you want to track which subset of the data you're working with, and how it changes by your operations.
- Software Engineers: When you have binary artifacts that you use as dependencies and would like to have a `make` alternative that considers _content changes_ rather than timestamps.
- Everyone: When you have photo, audio, document files to backup on Git, but don't want to copy that huge data to all Git clones. When you want to run a command when any member of these files change.

## ✳️ What is xvc for?

- (for x = files) Track large files on Git, store them on the cloud, retrieve when necessary, label
  and query for subsets
- (for x = pipelines) Define and run data -> model pipelines whose dependencies may be files,
  hyperparameters, regex searches, arbitrary URLs and more.
- (for x = experiments) Run isolated experiments, share them and store them in Git when necessary (TODO)
- (for x = data) Annotate data with arbitrary JSON and run queries and retrieve subsets of it. (TODO)
- (for x = models) Associate models with datasets, metadata and features, then track, store, and deploy them (TODO)

## 🔽 Installation

You can get the binary files for Linux, macOS and Windows from [releases] page. Copy the file to your path.

Alternatively, if you have Rust [installed], you can build xvc:

```shell
$ cargo install xvc
```

## Quick Start

Xvc tracks your files and directories on top of Git. To start run the following command in the repository.

```shell
$ xvc init
```

It initializes the metafiles in `.xvc/` directory and adds `.xvcignore` file in case you want to hide certain elements from Xvc.

Add your data files and directories for tracking.

```shell
$ xvc file track my-data/
$ git add .xvc
$ git commit -m "Began to track my-data/ with Xvc"
$ git push
```

The command calculates data content hashes (with BLAKE-3, by default) and records them.
It also copies files to content addressed directories under `.xvc/b3`

Define a file storage to share the files you added.

```shell
$ xvc remote new s3 --name my-remote --region us-east-1 --bucket-name my-xvc-remote
```

You can push the files you added to this remote.

```shell
$ xvc file push --to my-remote
```

You can now delete the files.

```shell
$ rm -r my-data/
```

When you want to access this data later, you can clone the repository and get back the files from file storage.

```shell
$ xvc file pull my-data/
```

If you have commands that depend on data or code elements, create a pipeline.

```shell
$ xvc pipeline new --name update-data
```

Then define a step in this pipeline to depend on other files.

```shell
$ xvc pipeline step new --name my-data-update --command 'python3 preprocess.py'
$ xvc pipeline step dependency --step my-data-update --files my-data/ --files preprocess.py
$ xvc pipeline step dependency --step my-data-update --regex 'names.txt:/^Name:'
$ xvc pipeline step dependency --step my-data-update --lines a-long-file.csv::-1000
$ xvc pipeline step output --step-name my-data-update --output-file preprocessed-data.npz
```

The above commands define a new step in the pipeline that depends on files in `my-data/` directory, and `preprocess.py`; lines that start with `Name:` in `names.txt`; and the first 1000 lines in `a-long-file.csv`. When _any_ of these conditions change, the step command (`python3 preprocess.py`) will run.

```shell
$ xvc pipeline run --name my-data-update
```

You can define fairly complex dependencies with globs, files, directories, regular expression searches in files, lines in files, other steps and pipelines with `xvc pipeline step dependency` commands. More dependency types like database queries, content from URLs, S3 (or compatible) buckets, Bitcoin (or other) wallets, REST and GraphQL results are in the backlog.

Please see [docs.xvc.ai] for documentation.

## ⚠️ Disclaimer

This software is fresh and ambitious. Although I use it and test it close to real world conditions, it didn't go under test of time yet. **Xvc can eat your files and spit it to eternal void!**

## 🤟 Big Thanks

xvc stands on the following (giant) crates:

- [serde] allows all data structures to be stored in text files. Special thanks from [`xvc-ecs`] for serializing components in an ECS with a single line of code.
- Xvc processes files in parallel with pipelines thanks to [crossbeam].
- Xvc uses [rayon] to calculate content hashes of millions of files in parallel.
- Thanks to [strum], Xvc uses enums extensively and converts almost everything to typed values from strings.
- Xvc has a deep CLI that has subcommands of subcommands like `xvc storage new s3`, and all these work with minimum bugs thanks to [clap].
- Xvc uses [rust-s3] to connect to S3 and compatible storage services.
- Without implementations of [BLAKE3], BLAKE2, SHA-2 and SHA-3 from Rust [crypto] crate, Xvc couldn't detect file changes so fast.
- Many thanks to small and well built crates, [reflink], [relative-path], [path-absolutize], [glob] and [wax] for file system and glob handling.
- Thanks to [sad_machine] for providing a State Machine implementation that I used in `xvc pipeline run`. State machine made running pipeline steps in parallel with a clean separation of process states.
- Thanks to [thiserror] and [anyhow] for making error handling a breeze. These two crates make me feel I'm doing something good for the humanity while handling errors.
- Xvc is split into many crates and owes this organization to [cargo workspaces].

[crossbeam]: https://docs.rs/crossbeam/latest/crossbeam/
[cargo workspaces]: https://crates.io/crates/cargo-workspaces
[rayon]: https://docs.rs/rayon/latest/rayon/
[strum]: https://docs.rs/strum/latest/strum/
[clap]: https://docs.rs/clap/4.0.18/clap/
[serde]: https://serde.rs
[blake3]: https://docs.rs/blake3/latest/blake3/
[crypto]: https://docs.rs/rust-crypto/latest/crypto/
[reflink]: https://docs.rs/reflink/0.1.3/reflink/
[relative-path]: https://docs.rs/relative-path/1.7.2/relative_path/
[path-absolutize]: https://docs.rs/path-absolutize/3.0.14/path_absolutize/
[glob]: https://docs.rs/glob/0.3.0/glob/
[wax]: https://docs.rs/wax/0.5.0/wax/
[sad_machine]: https://docs.rs/sad_machine/1.0.0/sad_machine/
[thiserror]: https://docs.rs/thiserror/latest/thiserror/
[anyhow]: https://docs.rs/anyhow/1.0.66/anyhow/
[rust-s3]: https://docs.rs/rust-s3/0.32.3/s3/
[`xvc-ecs`]: https://docs.rs/xvc-ecs/0.3.0/xvc_ecs/

And, biggest thanks to Rust designers, developers and contributors. Although I can't see myself expert to appreciate it all, it's a fabulous language and environment to work with.

## 🚁 Support

- You can use [Discussions](https://github.com/iesahin/xvc/discussions) to ask questions. I'll answer as much as possible. Thank you.
- For consultancy and paid support, you can [get in touch with me.](mailto:xvc@emresult.com).

## 👐 Contributing

- Star this repo. I feel very happy for five minutes for every star and send my best wishes to you.
- Really use xvc, tell me how it works for you, read the documentation, report bugs, dream about features. The greatest contribution might be this now.
- Write a new test with your workflow to increase testing coverage. They are under `lib/tests/` now but I'm planning to move them to a separate crate.
- [Be my guest](https://www.airbnb.com/users/show/3595069) when you visit Bursa. I usually don't have time to meet with every guest in person but if you let me know _you_ are coming, I'd like to arrange something. Also, when you visit Galata tower in İstanbul, which is close to where I live, you can buy me a coffee.
-
