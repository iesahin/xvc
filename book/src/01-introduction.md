# Introduction to Xvc

Xvc is a command line utility to track large files with Git, define dependencies between files to run commands when only these dependencies change, and run experiments by making small changes in these files for later comparison.
It's used mostly in Machine Learning scenarios where data and model files are large, code files depend on these and experiments must be compared via various metrics. 

Xvc can use S3 and compatible cloud storages to upload tracked files with their exact version and can retrieve these later. This allows to delete them from the project when they are not needed to save space and get them back when needed. This facility can also be used for sharing these files. You can just clone the Git repository and get only the necessary Xvc-tracked files.

Xvc tracks files, directories and other elements by calculating their digests. These digests are used as address to store and find their locations in the storages. When you make a change to a file, it gets a new digest and the changed version has a new address. This makes sure that all versions can be retrieved on demand. 

Xvc can be used as a `make` replacement to build multi-file projects with complex dependencies. Unlike `make` that detect file changes with timestamps, Xvc checks the files via their content. This reduces false-positives in invalidation.

Xvc pipelines are used to define steps to reach to a set of outputs. These steps have commands to run and may (or may not) produce intermediate outputs that other steps depend. Xvc pipelines allows steps to depend on other steps,  other pipelines, text and binary files, directories, globs that select a subset of files, certain lines in a file, certain regular expression results, URLs, (hyper)parameter definitions in YAML, JSON or TOML files as of now. More dependency types like environment variables, database tables and queries, S3 buckets, REST query results, generic CLI command results, Bitcoin wallets, Jupyter notebook cells are in the plans. 

For example, Xvc can be used to create a pipeline that depends on certain files in a directory via a glob, and a parameter in a YAML file to update a machine learning model. The same feature can be used to build software when the code or artifacts used in the software change. This allow binary outputs (as well as code inputs) to be tracked in Xvc. Instead of building everything from scratch in a new Git clone, a software project can reuse only the portions that require a rebuild. Binary distributions become much simpler. 

This book is used as the documentation of the project. It is a work in progress as Xvc, and contain outdated information. Please report any errors and bugs in https://github.com/iesahin/xvc as the rest of project. 

## Comparison with other tools

There are many similar tools for managing large files on Git, managing machine learning pipelines and experiments. Most of ML oriented tools are provided as SaaS and in a different vein than Xvc. 

Similar tools for file management on Git are the following: 

- `git-annex`: One of the earliest and most successful projects to manage large files on Git. It supports a large number of remote storage types, as well as adding other utilities as backends, similar to [`xvc storage generic`](./ref/xvc-storage-generic.md). It features an assistant aimed to make it easier for common use cases. It uses SHA-256 as the single digest option and uses symlinks as a [cache type.](./concepts/cache-types.md) It doesn't have data pipeline features. 
- `git-lfs`: It uses Git internals to track binary files. It requires server support for remote storages and allows only Git remotes to be used for binary file storage. Uses the same digest function Git uses. (By default, SHA-1). Uses `.gitattributes` mechanism to track certain files by default. It doesn't have data pipeline features.
- `dvc`: Uses YAML files _in the working directory_ to track file content. It uses MD5 sums. It can use different [cache type](./concepts/cache-types.md) for all the files in the repository. It has experiments tracking features, data pipelines and a [SaaS GUI.](https://studio.iterative.ai) 

I have done some preliminary benchmarks to measure _time to add_ files. I added 70.000 files with a single command. `xvc file track` (0.3.1) finished in 19 seconds, `git lfs track '*.png' ; git add 'data/images/**/*.png'` in 56 seconds, `dvc add data/images` in 80 seconds and `git-annex add data/images` in around 11 minutes. Note that these measurements are affected by output behavior and commands may gain some speed by turning off the default terminal output. Some finer benchmarks will be provided in the future, when Xvc is optimized. 


