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

## 🔽 Installation

You can get the binary files for Linux, macOS, and Windows from [releases](https://github.com/iesahin/xvc/releases/latest) page. Extract and copy the file to your `$PATH`.

Alternatively, if you have Rust [installed], you can build xvc:

```shell
$ cargo install xvc
```

[installed]: https://www.rust-lang.org/tools/install

If you want to use Xvc with Python console and Jupyter notebooks, you can also install it with `pip`:

```shell
$ pip install xvc
```

Note that pip installation doesn't make `xvc` available as a shell command. Please see [xvc.py](https://github.com/iesahin/xvc.py) for usage details.

## 🏃🏾 Quicktart

Xvc seamlessly monitors your files and directories on top of Git. To commence, execute the following command within the repository:

```console
$ git init # if you're not already in a Git repository
Initialized empty Git repository in [CWD]/.git/

$ xvc init
```

This command initializes the `.xvc/` directory and adds a `.xvcignore` file for specifying paths you wish to conceal from Xvc.

Include your data files and directories for tracking:

```shell
$ xvc file track my-data/ --as symlink
```

This command calculates content hashes for data (using BLAKE-3, by default) and logs them. The changes are committed to Git, and the files are copied to content-addressed directories within `.xvc/b3`. Additionally, read-only symbolic links to these directories are created.

You can specify different [recheck (checkout) methods](https://docs.xvc.dev/ref/xvc-file-recheck/) for files and directories, depending on your use case.
If you need to track model files that change frequently, you can set recheck method `--as copy` (the default).

```shell
$ xvc file track my-models/ --as copy
```

Configure a cloud storage to share the files you added.

```shell
$ xvc storage new s3 --name my-storage --region us-east-1 --bucket-name my-xvc-remote
```

You can send the files to this storage.

```shell
$ xvc file send --to my-storage
```

When you (or someone else) want to access these files later, you can clone the Git repository and get the files from the
storage.

```shell
$ git clone https://example.com/my-machine-learning-project
Cloning into 'my-machine-learning-project'...

$ cd my-machine-learning-project
$ xvc file bring my-data/ --from my-storage

```

This approach ensures convenient access to files from the shared storage when needed.

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

Suppose you're only interested in the IQ scores of those with _Dr._ in front of their names and how they differ from the rest in the dataset we created. Let's create a regex search dependency to the data file that will show all _doctors_ IQ scores.

```console
$ xvc pipeline step new --step-name dr-iq --command 'echo "${XVC_ADDED_REGEX_ITEMS}" >> dr-iq-scores.csv '
$ xvc pipeline step dependency --step-name dr-iq --regex-items 'random_names_iq_scores.csv:/^Dr\..*'
```

The first line specifies a command, when run writes `${XVC_ADDED_REGEX_ITEMS}` environment variable to `dr-iq-scores.csv` file.
The second line specifies the dependency which will also populate the `$[XVC_ADDED_REGEX_ITEMS]` environment variable in the command.

Some dependency types like [regex items],
[line items] and [glob items] inject environment variables in the commands they are a dependency.
For example, if you have two million files specified with a glob, but want to run a script only on the added files after the last run, you can use these environment variables.

When you run the pipeline again, a file named `dr-iq-scores.csv` will be created. Note that, as `requirements.txt` didn't change `install-deps` step and its dependent `generate-data` steps didn't run.

```console
$ xvc pipeline run
[DONE] dr-iq (echo "${XVC_ADDED_REGEX_ITEMS}" >> dr-iq-scores.csv )

$ cat dr-iq-scores.csv
Dr. Brian Shaffer,122
Dr. Brittany Chang,82
Dr. Mallory Payne MD,70
Dr. Sherry Leonard,93
Dr. Susan Swanson,81

```

We are using this feature to get lines starting with `Dr.` from the file and write them to another file. When the file changes, e.g. another record matching the dependency regex added to the `random_names_iq_scores.csv` file, it will also be added to `dr-iq-scores.csv` file.

```console
$ zsh -cl 'echo "Dr. Albert Einstein,144" >> random_names_iq_scores.csv'

$ xvc pipeline run
[DONE] dr-iq (echo "${XVC_ADDED_REGEX_ITEMS}" >> dr-iq-scores.csv )

$ cat dr-iq-scores.csv
Dr. Brian Shaffer,122
Dr. Brittany Chang,82
Dr. Mallory Payne MD,70
Dr. Sherry Leonard,93
Dr. Susan Swanson,81
Dr. Albert Einstein,144

```

Now we want to add a another command that draws a fancy histogram from `dr-iq-scores.csv`. As this new step must wait `dr-iq-scores.csv` file to be ready, we'll define `dr-iq-scores.csv` as an _output_ of `dr-iq` step and set the file as a dependency to this new `visualize` step.

```console
$ xvc pipeline step output --step-name dr-iq --output-file dr-iq-scores.csv
$ xvc pipeline step new --step-name visualize --command 'python3 visualize.py'
$ xvc pipeline step dependency --step-name visualize --file dr-iq-scores.csv
$ xvc pipeline run
[ERROR] Step visualize finished UNSUCCESSFULLY with command python3 visualize.py

```

You can get the pipeline in Graphviz DOT format to convert to an image.

```console
$ zsh -cl 'xvc pipeline dag | dot -opipeline.png'

```

You can also export and import the pipeline to JSON to edit in your editor.

```console
$ xvc pipeline export --file my-pipeline.json

$ cat my-pipeline.json
{
  "name": "default",
  "steps": [
    {
      "command": "python3 -m pip install --quiet --user -r requirements.txt",
      "dependencies": [
        {
          "File": {
            "content_digest": {
              "algorithm": "Blake3",
              "digest": [
                43,
                86,
                244,
                111,
                13,
                243,
                28,
                110,
                140,
                213,
                105,
                20,
                239,
                62,
                73,
                75,
                13,
                146,
                82,
                17,
                148,
                152,
                66,
                86,
                154,
                230,
                154,
                246,
                213,
                214,
                40,
                119
              ]
            },
            "path": "requirements.txt",
            "xvc_metadata": {
              "file_type": "File",
              "modified": {
                "nanos_since_epoch": [..],
                "secs_since_epoch": [..]
              },
              "size": 14
            }
          }
        }
      ],
      "invalidate": "ByDependencies",
      "name": "install-deps",
      "outputs": []
    },
    {
      "command": "python3 generate_data.py",
      "dependencies": [
        {
          "Step": {
            "name": "install-deps"
          }
        }
      ],
      "invalidate": "ByDependencies",
      "name": "generate-data",
      "outputs": []
    },
    {
      "command": "echo /"${XVC_ADDED_REGEX_ITEMS}/" >> dr-iq-scores.csv ",
      "dependencies": [
        {
          "RegexItems": {
            "lines": [
              "Dr. Brian Shaffer,122",
              "Dr. Susan Swanson,81",
              "Dr. Brittany Chang,82",
              "Dr. Mallory Payne MD,70",
              "Dr. Sherry Leonard,93",
              "Dr. Albert Einstein,144"
            ],
            "path": "random_names_iq_scores.csv",
            "regex": "^Dr//..*",
            "xvc_metadata": {
              "file_type": "File",
              "modified": {
                "nanos_since_epoch": [..],
                "secs_since_epoch": [..]
              },
              "size": 19021
            }
          }
        }
      ],
      "invalidate": "ByDependencies",
      "name": "dr-iq",
      "outputs": [
        {
          "File": {
            "path": "dr-iq-scores.csv"
          }
        }
      ]
    },
    {
      "command": "python3 visualize.py",
      "dependencies": [
        {
          "File": {
            "content_digest": null,
            "path": "dr-iq-scores.csv",
            "xvc_metadata": null
          }
        }
      ],
      "invalidate": "ByDependencies",
      "name": "visualize",
      "outputs": []
    }
  ],
  "version": 1,
  "workdir": ""
}
```

You can edit the file to change commands, add new dependencies, etc. and import it back to Xvc.

```console
$ xvc pipeline import --file my-pipeline.json --overwrite
```

Lastly, if you noticed that the commands are long to type, there is an `xvc aliases` command that prints a set of aliases for commands. You can source the output in your `.zshrc` or `.bashrc`, and use the following commands instead, e.g., `xvc pipelines run` becomes `pvc run`.

```console
$ xvc aliases

alias xls='xvc file list'
alias pvc='xvc pipeline'
alias fvc='xvc file'
alias xvcf='xvc file'
alias xvcft='xvc file track'
alias xvcfl='xvc file list'
alias xvcfs='xvc file send'
alias xvcfb='xvc file bring'
alias xvcfh='xvc file hash'
alias xvcfco='xvc file checkout'
alias xvcfr='xvc file recheck'
alias xvcp='xvc pipeline'
alias xvcpr='xvc pipeline run'
alias xvcps='xvc pipeline step'
alias xvcpsn='xvc pipeline step new'
alias xvcpsd='xvc pipeline step dependency'
alias xvcpso='xvc pipeline step output'
alias xvcpi='xvc pipeline import'
alias xvcpe='xvc pipeline export'
alias xvcpl='xvc pipeline list'
alias xvcpn='xvc pipeline new'
alias xvcpu='xvc pipeline update'
alias xvcpd='xvc pipeline dag'
alias xvcs='xvc storage'
alias xvcsn='xvc storage new'
alias xvcsl='xvc storage list'
alias xvcsr='xvc storage remove'

```

Please create an issue or discussion for any other kinds of dependencies that you'd like to be included.

I'm planning to add [data label and annotations tracking](https://github.com/iesahin/xvc/discussions/208)), [experiments tracking](https://github.com/iesahin/xvc/discussions/207)), [model tracking](https://github.com/iesahin/xvc/discussions/211)), encrypted cache, server to control all commands from a web interface, and more as my time permits.

Please check [`docs.xvc.dev`](https://docs.xvc.dev) for documentation.

## 🤟 Big Thanks

xvc stands on the following (giant) crates:

- [trycmd] is used to run all example commands in this file, [reference, and how-to documentation](https://docs.xvc.dev) at
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

- Star this repo. I feel very happy for every star and send my best wishes to you. That's a certain win to spend your two seconds for me. Thanks.
- Use xvc. Tell me how it works for you, read the [documentation](https://docs.xvc.dev), [report bugs](https://github.com/iesahin/xvc/issues), [discuss features](https://github.com/iesahin/xvc/discussions).
- Please note that, I don't accept large code PRs. Please open an issue to discuss your idea and write/modify a
  reference page before sending a PR. I'm happy to discuss and help you to implement your idea. Also, it may require a copyright transfer to me, as there may be cases which I provide the code in other licenses.

## 📜 License

Xvc is licensed under the [GNU GPL 3.0 License](https://github.com/iesahin/xvc/blob/main/LICENSE). If you want to use the code in your project with other licenses, please contact me.

## 🌦️ Future and Maintenance

I'm using Xvc daily and I'm happy with it. Tracking all my files with Git via arbitrary servers and cloud providers is
something I always need. I'm happy to improve and maintain it as long as I use it.

Given that I'm working on this for the last two years for pure technical bliss, you can expect me to work on it more.

## ⚠️ Disclaimer

This software is fresh and ambitious. Although I use it and test it close to real-world conditions, it didn't go under
the test of time. **Xvc can eat your files and spit them into the eternal void!** Please take backups.
