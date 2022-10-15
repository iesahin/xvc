# xvc

[![codecov](https://codecov.io/gh/iesahin/xvc/branch/master/graph/badge.svg?token=yrcNOA4RTy)](https://codecov.io/gh/iesahin/xvc)

An Fast and Robust MLOps Swiss-Army Knife in Rust

## What is xvc for?

- (for x = data) Track large files on Git, store them on the cloud, retrieve when necessary, label
   and query for subsets 
- (for x = pipelines) Define and run data -> model pipelines whose dependencies may be files,
   hyperparameters, regex searches, arbitrary URLs and more.
- (for x = experiments) Run isolated experiments, share them and store them in Git when necessary
- (for x = models) Associate models with datasets, metadata and features, then track & store them
  together

## Who is xvc for?

- Machine Learning Engineers: 
- Data Engineers: 
- Data Scientists:
- Software Engineers: 
- Everyone:

## Installation

## Quick Start

## Feature Comparisons

### DVC 

### Pachyderm: 

### git-annex:

### Git-LFS

## Benchmarks

## Thanks

xvc stands on the following (giant) crates: 

- [crossbeam]: Thanks for the channels. I could create decent and clean parallel pipelines with them. 
- [cargo workspaces]: I'm able to track the crate versions, and bump them with a single command. 
- [rayon]: I ❤️  parallel iterators. 
- [strum]:
- [clap]:
