# Xvc for DVC Users

[DVC](https://dvc.org) is an MLOps utility to track data, pipelines and machine
learning experiments on top of Git. Xvc is inspired by DVC in its purpose, but
there are major technical differences between these two.

Note that this document refers mostly to Xvc v0.6 and DVC 2.30.
Both commands are in development, and similarities and differences may change in time.

## Similarities

The purposes of these two commands are similar, and these are alternatives to
each other. Both of these aims to manage data, pipelines and experiments of an
ML project.

Both of the utilities similarly work on top of Git. DVC became more bound to
Git after the introduction of its experiment tracking features. Before that,
Git was optional (but recommended) for DVC.

Xvc has the same optional and recommended reliance on Git but all features are
available without Git.

Both of these commands use hashing the content to detect changes in files.

Both of these use DAGs to represent pipelines.

## Conceptual Differences

- What DVC calls "remote", Xvc calls "storage." This is to emphasize the
  difference between Xvc storages and Git remotes.

- What DVC calls "stage" in a data pipeline, Xvc calls "step." "Stage" has a
  different meaning in the Git context, and I believe using the same word in a
  different meaning increases the mental effort to describe and understand.

- In DVC, there is a 1-1 correspondence between `dvc.yaml` files in a
  repository and the pipelines. In Xvc, pipelines are more abstract. They are
  defined with [`xvc pipeline`](/ref/xvc-pipeline.md) family of commands. No
  single file contains a pipeline definition. You can [export
  pipelines](/ref/xvc-pipeline-export.md) to YAML, JSON, and TOML, and [import
  them](/ref/xvc-pipeline-import.md) after making changes. Xvc doesn't consider
  any file format authoritative for pipelines, and their YAML/JSON/TOML
  representation may change between versions.

- DVC is more liberal in creating files among user files in the repository.
  When you add a file to DVC with `dvc add`, DVC creates a `.dvc` file next to
  it. Xvc only creates a `.xvc/` directory in the repository root and only
  updates `.gitignore` files to hide tracked files from Git. You won't see any
  files added next to your data files.

- Cache type, (or rather [recheck](/concepts/recheck.md) method) that is
  whether a file in the repository is linked to its cached version by copying,
  reflink, symlink or hardlink is determined repository-wide in DVC. You can
  either have all your cache links as symlinks, or hardlinks, etc. Xvc tracks
  these per file, you can have one file symlinked to the cache, another file
  copied from the cache, etc.

## Command Differences

❗Some of the Xvc commands described here are still in development.

- While naming Xvc commands, I tried our best to avoid name clashes with Git.
  Having both `git push` and `dvc push` commands may look beneficial for
  description at first, as these two are analogous. However, giving the same name
  also hides important details that are more difficult to emphasize later. (e.g.
  DVC experiments are _Git objects_ that are pushed to _Git remotes_, while the
  files changed during experiments are pushed to _DVC remotes._)

- `dvc add` can be replaced by [`xvc file track`](/ref/xvc-file-track.md). `dvc
add` creates a `.dvc` file (formatted in YAML) in the repository. Xvc doesn't
  create separate files for tracked paths.

- Instead of deleting `.dvc` files to remove a file from DVC, you can use [`xvc
file untrack`](/ref/xvc-file-untrack.md). It can also restore all versions of
  an untracked file to a directory.

- `dvc check-ignore` can be replaced by `xvc check-ignore`. Xvc version can be
  used against any other ignore filename. (`.gitignore`,`.ignore`,
  `.fooignore`...)

- `dvc checkout` is replaced by [`xvc file recheck`](/ref/xvc-file-recheck.md).
  There is a `--recheck-method` (shortened as `--as`) option in several Xvc
  commands to tell whether to check out as symlink, hardlink, reflink or copy.

- `dvc commit` is replaced by [`xvc file carry-in`](/ref/xvc-file-carry-in).

- There is no command similar to `dvc config`. You can either edit the
  [configuration files](/intro/configuration.md), or modify configuration with
  `-c` options in each run. You can also supply all configuration from the
  environment. See [Configuration](/intro/configuration.md).

- `dvc dag` is replaced by `xvc pipeline dag`. DVC version uses ASCII art to
  present the pipeline. Xvc doesn't provide ASCII art, instead provides either a
  Graphviz representation or mermaid diagram.

- `dvc data status` and `dvc status` can be replaced by `xvc file list`. Xvc
  version doesn't provide information about the pipelines, or remote storages.

- There is no command similar to `dvc destroy` in Xvc. There will be an [`xvc
deinit`](/ref/xvc-deinit.md) command at some point.

- There is no command similar to `dvc diff` in Xvc.

- There is no command similar to `dvc doctor` or `dvc version`. Version
  information should be visible in the help text. Unless [compiled from source
  with feature flags](/intro/install.md), Xvc binaries don't have feature
  differences.

- Currently, there are no commands corresponding to `dvc exp` set of commands.
  This is on the roadmap for Xvc. Scope, implementation, and actual commands may
  differ.

- `dvc fetch` is replaced by [`xvc file bring
--no-recheck`](/ref/xvc-file-bring.md).

- Instead of freezing "pipeline stages" as in `dvc freeze`, and unfreezing with
  `dvc unfreeze`, `xvc pipeline step update --changed
[never|always|by_dependencies]` can be used to specify if/when to run a
  pipeline step.

- Instead of `dvc gc` to "garbage-collect" files, you can use [`xvc file
remove`](/ref/xvc-file-remove.md) with various options.

- There is no corresponding command for `dvc get-url` in Xvc. You can use
  `wget` or `curl` instead.

- Currently there is no command to replace `dvc get` and `dvc import`, and `dvc
import-url`. URL dependencies are supported in the pipeline with `xvc pipeline step dependency --url`.

- Instead of `dvc install` like hooks, Xvc issues Git commands itself if
  `git.auto_commit` , `git.auto_stage` configuration options are set.

- There is no corresponding command for `dvc list-url`.

- `dvc list` is replaced by [`xvc file list`](/ref/xvc-file-list.md) for local
  paths. Its remote capabilities are not implemented but is on the roadmap.

- Xvc doesn't mix files from different repositories in the same storage. There
  is an ID for each Xvc repo that's also used in remote storage paths.

- Currently, there is no params/metrics tracking/diff similar to `dvc params`,
  `dvc metrics` or `dvc plots` commands in Xvc.

- `dvc move` is replaced by [`xvc file move`](/ref/xvc-file-move.md).

- `dvc push` is replaced by [`xvc file send`](/ref/xvc-file-send.md).

- `dvc pull` is replaced by [`xvc file bring`](/xvc-file-bring.md).

- There are no commands similar to `dvc queue` for experiments in Xvc.
  Experiment tracking will probably be handled differently.

- `dvc remote` set of commands are replaced by `xvc storage` set of commands.
  You can use `xvc storage new` for adding new storages. Currently, there is no
  "default remote" facility in Xvc. Instead of `dvc remote modify`, you can use
  `xvc storage remove` and `xvc storage new`.

- There is no single command to replace `dvc remove`. For files, you can use
  [`xvc file delete`](/ref/xvc-file-delete.md). For pipelines steps, you can use
  ][`xvc pipeline step remove`](/ref/xvc-pipeline-step-remove.md)

- Instead of `dvc repro`, Xvc has [`xvc pipeline
run`](/ref/xvc-pipeline-run.md). If you want to reproduce a pipeline, you can
  use `xvc pipeline run` again.

- `xvc root` is for the same purpose as `dvc root`.

- `dvc run` (that defines a stage in DVC pipeline and immediately runs it) can
  be replaced by [`xvc pipeline`](/ref/xvc-pipeline.md) set of commands. [`xvc
pipeline new`](/ref/xvc-pipeline-new.md) for a new pipeline, [`xvc pipeline
step new`](/ref/xvc-pipeline-step-new.md) for a new step in the pipeline, [`xvc
pipeline step dependency`](/ref/xvc-pipeline-step-dependency.md) to specify
  dependencies of a step, [`xvc pipeline step
output`](/ref/xvc-pipeline-step-output.md) to specify outputs of a step and
  [`xvc pipeline run`](/ref/xvc-pipeline-run.md) to run this pipeline.

- Instead of `dvc stage add`, we have [`xvc pipeline step
new`](/ref/xvc-pipeline-step-new.md). For `dvc stage list`, we have [`xvc
pipeline step list`](/ref/xvc-pipeline-step-list.md).

- There is no (need) for `dvc protect` or `dvc unprotect` commands in Xvc.
  "Cache type" of DVC is not a repository-wide option, and called ["recheck
  method"](/concepts/recheck.md). If you want to track a certain directory as
  symlink, and another as hardlink, you can do so with `xvc file recheck --as`.
  If you want identical files copied to one directory _and_ linked in another,
  [`xvc file copy`](/ref/xvc-file-copy.md) can help.

- DVC needs `dvc update` for external dependencies in pipelines. Xvc checks
  their metadata like any other dependency before downloading and invalidates the
  step if the URL/file has changed automatically.

- DVC leaves Git operations to the user, and automates them to a certain degree
  with Git hooks. Xvc adds Git commits to the repository after operations by
  default.

## Technical Differences

- DVC is written in Python. Xvc is written in Rust.

- DVC uses MD5 to check file content changes. Xvc uses BLAKE3 by default, and
  can be configured to use BLAKE2s, SHA2-256 and SHA3-256.

- DVC tracks file/directory changes in separate `.dvc` files. Xvc tracks them
  in `.json` files in `.xvc/store`. There is no 1-1 correspondence between these
  files and the directory structure.

- DVC uses Object-Oriented Programming in Python. Xvc tries to minimize
  function/data coupling and uses an Entity-Component System (`xvc-ecs`) in its
  core.

- DVC remotes are identical to their cache in structure, and multiple DVC
  repositories use the same remote by mixing files. This provides
  inter-repository deduplication. Xvc uses separate directory for each
  repository. This means identical files in separate Xvc repositories are
  duplicated and when you want to delete all files associated with a repository,
  you can do so without the risk of deleting files used in other repositories.

- DVC considers directories as file-equivalent entities to track with `.dvc`
  files pointing to `.json` files in the cache. Xvc doesn't track directories as
  identical to files. They are considered collections of files.

- DVC uses [Dulwich](https://www.dulwich.io) for Git operations. Xvc [executes
  the Git process directly](/arch/git-and-xvc.md), with its common command line
  options.
