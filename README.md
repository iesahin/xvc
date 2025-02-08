# xvc

[![codecov](https://codecov.io/gh/iesahin/xvc/branch/main/graph/badge.svg?token=xa3ru5KhRq)](https://codecov.io/gh/iesahin/xvc)
[![build](https://img.shields.io/github/actions/workflow/status/iesahin/xvc/rust.yml?branch=main)](https://github.com/iesahin/xvc/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/xvc)](https://crates.io/crates/xvc)
[![docs.rs](https://img.shields.io/docsrs/xvc)](https://docs.rs/xvc/)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

Manage your unstructured data next to code in Git repositories and run commands when they change. 

## ⌛ Why Xvc?

- You have image, audio, media, document or asset files to [track/version/backup][xvc-file-track] along with the code, but [don't want to copy][xvc-file-recheck] that huge data to all Git clones.
- You want to [manage][xvc-file-list] unstructured data in multiple places with
[multiple subsets][xvc-file-copy], some (e.g. data) being read-only and some
(e.g. models, executables) change frequently. 
- You want to [store][xvc-storage-new] this data in [local][xvc-storage-new-local], [SSH-accessible][xvc-storage-new-rsync], or [S3-compatible cloud storages][xvc-storage-new-s3] to share along the repository. 
- You want to [specify commands][xvc-pipeline-step-new] that [run][xvc-pipeline-run] when only input data changes, define [pipelines][xvc-pipeline-new] with steps that run when only their [dependencies][xvc-pipeline-step-dependency] change.
  - You want to define these dependencies with [files][xvc-pipeline-step-dependency-file], [globs][xvc-pipeline-step-dependency-glob] spanning multiple files, text file lines defined by [ranges][xvc-pipeline-step-dependency-line] or [regexes][xvc-pipeline-step-dependency-regex], [URLs][xvc-pipeline-step-dependency-url], [parameters][xvc-pipeline-step-dependency-params] in the YAML or JSON files, [SQLite queries][xvc-pipeline-step-dependency-sqlite]
  or [any command][xvc-pipeline-step-dependency-generic] that produces output. 

### ✅ Common Tasks 

<details>
  <summary> <strong> 🔽 Installation</strong></summary>

You can get the binary files for Linux, macOS, and Windows from
[releases](https://github.com/iesahin/xvc/releases/latest) page. Extract and
copy the file to your `$PATH`.

Alternatively, if you have Rust [installed], you can build xvc:

```shell
$ cargo install xvc
```

[installed]: https://www.rust-lang.org/tools/install

If you want to use Xvc with Python console and Jupyter notebooks, you can also
install it with `pip`:

```shell
$ pip install xvc
```

Note that pip installation doesn't make `xvc` available as a shell command.
Please see [xvc.py] for details.

[xvc.py]: https://github.com/iesahin/xvc.py

### Completions

Xvc supports dynamic completions for bash, zsh, elvish, fish and powershell. For example, run the following to add completions for bash:

```bash
echo "source <(COMPLETE=bash xvc)" >> ~/.bashrc
```

See [completions] section in the docs for others.

[completions]: https://docs.xvc.dev/intro/completions

</details>

<details>
  <summary>🚀
    <strong> Initialize a directory for Xvc</strong>
  </summary>

```console
$ git init # if you're not already in a Git repository
Initialized empty Git repository in [CWD]/.git/

$ xvc init
```

This command initializes the `.xvc/` directory and adds a `.xvcignore` file for specifying paths you wish to hide from Xvc.

  > 💡**Tip**:
  > Git is **not required** to run Xvc. However running Xvc with Git is usually
  > a good idea. Xvc can stage/commit metadata files (under `.xvc/`) used to
  > track binary files and you can use branches for versioning as well. By
  > default, you won't have to deal with Git commands to commit these metadata
  > files.
  > 
  > If you don't want to use Xvc with Git, use `--no-git` option when
  > initializing.

</details>

<details>
  <summary>
    👣
    <strong>Add Files for Tracking</strong>
  </summary>

Include your data files and directories for tracking:

```shell
$ xvc file track my-data/
```

[This command](https://docs.xvc.dev/ref/xvc-file-track.html) calculates content
hashes for data (using BLAKE-3, by default) and records them. Files are moved
to content-addressed directories under `.xvc/b3`. Then they are copied to the
workspace. 

  > 💡**Tip**:
  > You can specify different [recheck (checkout)
  > methods](https://docs.xvc.dev/ref/xvc-file-recheck/) for files and
  > directories depending on your use case. Symlinks and hardlinks to the
  > files under Xvc cache don't consume additional space but they are readonly.
  > You can also use (copy-on-write) reflinks if your file system supports it
  > and Xvc is built with `reflink` feature. 

</details>

<details>
<summary>🫧 
    <strong>Checkout a subset of files as symlinks</strong>
</summary>

You can copy and recheck (checkout) subsets of files from Xvc cache as symlinks
to create multiple _views_. This is useful when you need a read-only access
that won't consume additional space.

```console
$ xvc file copy my-data/ another-view-to-my-data/
$ xvc file recheck another-view-to-my-data/ --as symlink
```
  > 💡**Tip**:
  > [`xvc file copy`][xvc-file-copy] and [`xvc file move`][xvc-file-move]
  > doesn't require file contents to be available. Xvc works only with their
  > metadata and you can organize files without their content copied to
  > workspace or cache. 
  
  > 💡**Tip**:
  > If you installed [completions] to your shell, Xvc completes file names even
  > if they are not available in the workspace. 

</details>

<details>
<summary> 🌁 <strong>Send files to the cloud services</strong></summary>

Configure a cloud storage to share the files you track with Xvc.

```shell
$ xvc storage new s3 --name my-storage --region us-east-1 --bucket-name xvc
```

You can send the files to this storage.

```shell
$ xvc file send --to my-storage
```

You can also send a subset of the files.

```shell
$ xvc file send 'my-data/training/*' --to my-storage
```

Xvc [supports](https://docs.xvc.dev/ref/xvc-storage-new) [external directories](https://docs.xvc.dev/ref/xvc-storage-new-local), [Rsync](https://docs.xvc.dev/ref/xvc-storage-new-rsync), [AWS S3](https://docs.xvc.dev/ref/xvc-storage-new-s3), [Google Cloud Storage](https://docs.xvc.dev/ref/xvc-storage-new-gcs), [MinIO](https://docs.xvc.dev/ref/xvc-storage-new-minio), [Cloudflare R2](https://docs.xvc.dev/ref/xvc-storage-new-r2), [Wasabi](https://docs.xvc.dev/ref/xvc-storage-new-wasabi), [Digital Ocean Spaces](https://docs.xvc.dev/ref/xvc-storage-new-digital-ocean). Please [create an issue](https://github.com/iesahin/xvc/issues?q=sort%3Aupdated-desc+is%3Aissue+is%3Aopen) if you want Xvc to support another cloud storage service.

> 💡**Tip**:
> Xvc also supports any command to upload/download files. If your favorite
> service is not listed or you want to use another tool (s5cmd, rclone, etc.),
> you can specify a [generic](https://docs.xvc.dev/ref/xvc-storage-new-generic)
> storage by supplying shell commands to upload and download. 

> 📌 **Important**:
> Xvc never stores credentials to your connections and expects them to be
> available in the environment. It _never_ makes network requests (for
> tracking, statistics, etc.) without your knowledge. You can
> [compile](https://docs.xvc.dev/intro/compile-without-default-features)
> without cloud connection support in case you want to make sure that it
> makes no connections to outside services.

</details>

<details>
  <summary> 🪣 
    <strong>Get Files from cloud services</strong>
  </summary>

When you (or someone else) want to access these files later, you can clone the
Git repository and [get the files][xvc-file-bring] from the storage.

```shell
$ git clone https://example.com/my-machine-learning-project
Cloning into 'my-machine-learning-project'...

$ cd my-machine-learning-project
$ xvc file bring my-data/ --from my-storage

```

This approach ensures convenient access to files from the shared storage when
needed.

  > 💡**Tip**:
  > You don't have to reconfigure the storage after cloning, but you need to
  > have valid credentials as environment variables to access the storage. Xvc
  > never stores any credentials.

</details>

<details>
  <summary> 🫖
    <strong>Share files from cloud storages for a limited time</strong> 
  </summary>
  
  You can share Xvc tracked files from S3 compatible storages for a specified period.

```shell
$ xvc file share --storage my-storage dir-0001/file-0001.bin --duration 1h
https://my-storage.s3.eu-central-1.amazonaws.com/xvc....
```

You can share the link with others and they will be able to access to the file hour. The default period is 24 hours.

</details>

<details>
<summary> 🥤<strong>Create a data pipeline</strong></summary>

Suppose you have a script to preprocess files in a directory and you want to run this when the files in `my-data/train` directory changes. We first define a step in the pipeline that will run the script.

```console
$ xvc pipeline step new --step-name install-deps --command 'python3 src/preprocess.py'
```

Each command is associated with a step and each step has a command.

</details>

<details>
<summary> 🔗 <strong>Add a dependency to a pipeline step</strong></summary>

When we want to create a dependency for a command, we use 

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
</details>

<details>
<summary> 🛝 <strong>Run pipeline</strong></summary>

After you define the pipeline, you can run it by:

```console
$ xvc pipeline run
[DONE] install-deps (python3 -m pip install --quiet --user -r requirements.txt)
[OUT] [generate-data] CSV file generated successfully.

[DONE] generate-data (python3 generate_data.py)

```

</details>

<details>
<summary> 🪡 <strong>Add fine grained dependencies to steps</strong></summary>

Xvc allows many kinds of dependencies, like [files](https://docs.xvc.dev/ref/xvc-pipeline-step-dependency#file-dependencies),
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
</details>


<details>
<summary> 🎋 <strong>Visualize a pipeline in Graphviz or Mermaid</strong></summary>

You can get the pipeline in Graphviz DOT format to convert to an image.

```console
$ zsh -cl 'xvc pipeline dag --format graphviz | dot -opipeline.png'

```

You can also ask for a [mermaid.js]() diagram;


```console
xvc pipeline dag --format mermaid
```

</details>

<details>
<summary> 🐾 <strong>Export a pipeline in YAML or JSON format</strong></summary>
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
</details>

<details>
  <summary> 🛃 <strong>Import a pipeline from JSON or YAML files</strong></summary>

```console
$ xvc pipeline import --file my-pipeline.json --overwrite
```

</details>

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


[xvc-file-track]: https://docs.xvc.dev/ref/xvc-file-track
[xvc-file-list]: https://docs.xvc.dev/ref/xvc-file-list
[xvc-file-recheck]: https://docs.xvc.dev/ref/xvc-file-recheck
[xvc-file-send]: https://docs.xvc.dev/ref/xvc-file-send
[xvc-file-bring]: https://docs.xvc.dev/ref/xvc-file-bring
[xvc-file-copy]: https://docs.xvc.dev/ref/xvc-file-copy
[xvc-file-move]: https://docs.xvc.dev/ref/xvc-file-move
