# xvc init


## Synopsis 

```console
$ xvc init --help
Initialize an Xvc project

Usage: xvc init [OPTIONS]

Options:
      --path <PATH>  Path to the directory to be intialized. (default: current directory)
      --no-git       Don't require Git
      --force        Create the repository even if already initialized. Overwrites the current .xvc directory Resets all data and guid, etc
  -h, --help         Print help information

```

## Examples

To initialize a blank Xvc repository, initialize Git first and run `xvc init`. 

```console
$ cd my-project-1
$ git init
...
$ xvc init
? 0
```

The command doesn't print anything upon success.

If you want to initialize 
