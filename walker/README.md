# xvc-walker

[![Crates.io](https://img.shields.io/crates/v/xvc-walker.svg)](https://crates.io/crates/xvc-walker)
[![Documentation](https://docs.rs/xvc-walker/badge.svg)](https://docs.rs/xvc-walker)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

A high-performance file system walker with `.gitignore`-like ignore rule handling for Rust projects.

## Overview

`xvc-walker` provides functionality to traverse directory trees efficiently while respecting ignore rules similar to [.gitignore](https://git-scm.com/docs/gitignore). It can be used to efficiently scan large directory structures with both parallel and serial options.

## Features

- **Efficient File Traversal**: Walks directory trees with optimized performance
- **Parallel Processing**: Uses rayon for parallelized directory traversal
- **Ignore Rules**: Supports `.gitignore`-style patterns to skip files and directories
- **File System Events**: Integration with `notify` for watching file system changes
- **Flexible Configuration**: Customizable walking behavior with options

## Installation

Add this to your `Cargo.toml`:
