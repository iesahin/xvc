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

```console
$ git init # if you're not already in a Git repository
Initialized empty Git repository in [CWD]/.git/

$ xvc init
```

It initializes the metafiles in `.xvc/` directory and adds `.xvcignore` file for paths you may want to hide from Xvc.

Add your data files and directories for tracking.

```shell
$ xvc file track my-data/ --as symlink
```

The command calculates data content hashes (with BLAKE-3, by default) and records them.
It commits these changes to Git.
It also copies these files to content-addressed directories under `.xvc/b3` and creates read-only symbolic links to them.

You can specify different [recheck (checkout) methods](https://docs.xvc.dev/ref/xvc-file-recheck/) for files and directories, depending on your use case.
If you need to track model files that change frequently, you can set recheck method `--as copy` (the default).

```shell
$ xvc file track my-models/ --as copy
```

Configure a cloud storage to share the files you added.

```shell
$ xvc storage new s3 --name my-remote --region us-east-1 --bucket-name my-xvc-remote
```

You can send the files to this storage.

```shell
$ xvc file send --to my-remote
```

When you (or someone else) want to access these files later, you can clone the Git repository and get the files from the
storage.

```shell
$ git clone https://example.com/my-machine-learning-project
Cloning into 'my-machine-learning-project'...

$ cd my-machine-learning-project
$ xvc file bring my-data/ --from my-remote

```

You don't have to reconfigure the storage after cloning, but you need to have valid credentials as environment variables
to access the storage.
Xvc never stores any credentials.

If you have commands that depend on data or code elements, you can configure a pipeline.

For this example, we'll use [a Python script](https://github.com/iesahin/xvc/blob/main/workflow_tests/templates/README.in/generate_data.py) to generate a data set with random names with random IQ scores.

The script uses the Faker library and this library must be available where you run the pipeline. To make it repeatable, we start the pipeline by adding a step that installs dependencies.

```console
$ xvc pipeline step new --step-name install-deps --command 'python3 -m pip install --quiet --user -r requirements.txt'
```

We'll make this this step to depend on `requirements.txt` file, so when the file changes it will make the step run. 

```console
$ xvc pipeline step dependency --step-name install-deps --file requirements.txt
```

Xvc allows to create dependencies between pipeline steps. Dependent steps wait for dependencies to finish successfully. 

Now we create a step to run the script and make `install-deps` step a dependency of it. 

```console
$ xvc pipeline step new --step-name generate-data --command 'python3 generate_data.py'
$ xvc pipeline step dependency --step-name generate-data --step install-deps
```

After you define the pipeline, you can run it by:

```console
$ xvc pipeline run
[DONE] install-deps (python3 -m pip install --quiet --user -r requirements.txt)
[OUT] [generate-data] CSV file generated successfully.
 
[DONE] generate-data (python3 generate_data.py)

```

Xvc allows many kinds of dependnecies, like [files](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#file-dependencies), 
[groups of files and directories defined by globs](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#glob-dependencies), 
[regular expression searches in files](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#regex-dependencies), 
[line ranges in files](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#line-dependencies), 
[hyper-parameters defined in YAML, JSON or TOML files](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#hyper-parameter-dependencies)
[HTTP URLs](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#url-dependencies),
[shell command outputs](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#generic-command-dependencies), 
and [other steps](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#step-dependencies). 

Suppose you're only interested in the IQ scores of Greg's and how they differ from the rest in the dataset we created above. Let's create a regex search dependency to the data file that will show all Greg's IQ scores. 

```console
$ xvc pipeline step new --step-name greg-iq --command 'echo "${XVC_REGEX_ALL_ITEMS}" > greg-iq-scores.csv '
$ xvc pipeline step dependency --step-name greg-iq --regex-items 'random_names_iq_scores.csv:/.*Greg.*'
```

When you run the pipeline again, a file named `greg-iq-scores.csv` will be created. Note that, as `requirements.txt` didn't change `install-deps` step and its dependent `generate-data` steps didn't run.

```console
$ xvc pipeline run
[DONE] greg-iq (echo "${XVC_REGEX_ALL_ITEMS}" > greg-iq-scores.csv )

$ cat greg-iq-scores.csv


````

You can get the pipeline in Graphviz DOT format to convert to an image.

```console
$ xvc pipeline dag
digraph pipeline{n0[shape=box;label="install-deps";];n1[shape=note;label="requirements.txt";];n0->n1;n2[shape=box;label="generate-data";];n0[shape=box;label="install-deps";];n2->n0;n3[shape=box;label="greg-iq";];n4[shape=signature;label="random_names_iq_scores.csv:/.*Greg.*";];n3->n4;}

```

You can also export and import the pipeline to JSON to edit in your editor.

```console
$ xvc pipeline export > my-pipeline.json
? 2
error: unexpected argument '>' found

Usage: xvc pipeline export [OPTIONS]

For more information, try '--help'.

$ nvim my-pipeline.json
$ xvc pipeline import --file my-pipeline.json --overwrite
[ERROR] Pipeline Error: I/O Error: No such file or directory (os error 2)

```

You can run the pipeline with.

```shell
$ xvc pipeline run
```

If the steps you defined doesn't depend to each other, they are run in parallel.

You can define fairly complex dependencies with globs, files, directories, regular expression searches in files, lines in files, other steps and pipelines with `xvc pipeline step dependency` commands.
More dependency types like database queries, content from URLs, S3 (and compatible) buckets, REST and GraphQL results are in the backlog.
Please create an issue or discussion for any other kinds of dependencies that you'd like to be included.

Please check [`docs.xvc.dev`](https://docs.xvc.dev) for documentation.

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

I'm using Xvc daily and I'm happy with it. Tracking all my files with Git via arbitrary servers and cloud providers is
something I always need. I'm happy to improve and maintain it as long as I use it.

Also, I'm applying my technical/architectural ideas to see their effectiveness
and I have more ideas to implement. I don't expect to be bored from this soon.

I'm in a phase of my life where material success doesn't entice me.
I have a daily routine that I love, which includes spending 1-2 hours to Xvc.
That probably won't change much even if I earn a billion dollars.
I don't want to convert Xvc to a business.
I may create paid apps that use Xvc as a library if I think they will be useful but these will probably be separate projects.
In my opinion, trying to monetize FOSS prematurely deteriorates it more than other factors.

Xvc is like a _runnable CV_ for me. It signals _I built this and I can built similar software for you._ This is
another motivation for me to keep it alive. I hate updating my [vita](https://emresahin.net/cv/), and instead of
updating it, I prefer to show my work.

## ⚠️ Disclaimer

This software is fresh and ambitious. Although I use it and test it close to real-world conditions, it didn't go under
the test of time. **Xvc can eat your files and spit them into the eternal void!** Please take backups.
