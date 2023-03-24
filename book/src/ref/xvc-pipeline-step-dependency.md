# xvc pipeline step dependency

## Purpose

Define a dependency to an existing step in the pipeline.

## Synopsis

```console
$ xvc pipeline step dependency --help
Add a dependency to a step

Usage: xvc pipeline step dependency [OPTIONS] --step-name <STEP_NAME>

Options:
  -s, --step-name <STEP_NAME>    Name of the step to add the dependency to
      --generic <GENERICS>       Add a generic command output as a dependency. Can be used multiple times. Please delimit the command with ' ' to avoid shell expansion
      --url <URLS>               Add a URL dependency to the step. Can be used multiple times
      --file <FILES>             Add a file dependency to the step. Can be used multiple times
      --step <STEPS>             Add a step dependency to a step. Can be used multiple times. Steps are referred with their names
      --directory <DIRECTORIES>  Add a directory dependency to the step. Can be used multiple times
      --glob <GLOBS>             Add a glob dependency to the step. Can be used multiple times
      --param <PARAMS>           Add a parameter dependency to the step in the form filename.yaml::model.units . Can be used multiple times
      --regex <REGEXPS>          Add a regex dependency in the form filename.txt:/^regex/ . Can be used multiple times
      --line <LINES>             Add a line dependency in the form filename.txt::123-234
  -h, --help                     Print help

```

## Examples

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

Begin by adding a new step.

```console
$ xvc pipeline step new --step-name file-dependency --command "echo data.txt has changed"
```

Add a file dependency to the step.

```console
$ xvc pipeline step dependency --step-name file-dependency --file data.txt
```

When you run the command, it will print `data.txt has changed` if the file `data.txt` has changed.

```console
$ xvc pipeline run

```

You can add multiple dependencies to a step with multiple invocations.

```console
$ xvc pipeline step dependency --step-name file-dependency --file data2.txt
```

A step will run if any of its dependencies have changed.

```console
$ xvc pipeline run

```

Normally, they are not run if none of the dependencies have changed.

```console
$ xvc pipeline run

```

However, if you want to run the step even if none of the dependencies have changed, you can set the `--when` option to `always`.

```console
$ xvc pipeline step update --step-name file-dependency --when always
```

Now the step will run even if none of the dependencies have changed.

```console
$ xvc pipeline run
[OUT] data.txt has changed

[OUT] [EXIT] Successfully

```

### Step Dependencies

You can add a step dependency to a step. These steps specify dependency relationships explicitly, without relying on
changed files or directories.

```console
$ xvc pipeline step new --step-name world --command "echo world"
$ xvc pipeline step new --step-name hello --command "echo hello"
$ xvc pipeline step dependency --step-name world --step hello
$ xvc pipeline step dependency --step-name hello --step file-dependency
```

When run, the dependency will be run first and the step will be run after.

```console
$ xvc pipeline run
[OUT] data.txt has changed

[OUT] [EXIT] Successfully

```

### Generic Command Dependencies

You can use the output of a command as a dependency to a step. When the command is run, the output hash is saved to
compare and to invalidate the step when the output has changed.

You can use this for any command that outputs a string.

```console
$ xvc pipeline new --name generic

$ xvc pipeline --name generic step new --step-name yearly --command "echo 'Happy New Year! Welcome `(date +%Y)`!'"

$ xvc  pipeline --name generic step dependency --step-name yearly --generic 'date +%Y'

```

```console
$ xvc pipeline --name generic export
{
  "name": "generic",
  "steps": [
    {
      "command": "echo 'Happy New Year! Welcome `(date +%Y)`!'",
      "dependencies": [
        {
          "Generic": {
            "generic_command": "date +%Y"
          }
        }
      ],
      "invalidate": "ByDependencies",
      "name": "yearly",
      "outputs": []
    }
  ],
  "version": 1,
  "workdir": ""
}

```

When the year changes, the step is invalidated and run again.

```console
$ xvc pipeline --name generic run
[OUT] Happy New Year! Welcome `(date +%Y)`!

[OUT] [EXIT] Successfully

```

The step won't run until the next year.

```console
$ xvc pipeline --name generic run

```

### Directory Dependencies

You can specify a directory in the Xvc repository as a dependency to a step. When the directory changes, the step is
invalidated and run again.

We'll run the following commands in the `examples` directory.

```console
$ xvc-test-helper create-directory-tree --directories 2 --files 3 --seed 20230323
$ tree
.
├── dir-0001
│   ├── file-0001.bin
│   ├── file-0002.bin
│   └── file-0003.bin
└── dir-0002
    ├── file-0001.bin
    ├── file-0002.bin
    └── file-0003.bin

3 directories, 6 files

```

```console
$ xvc pipeline new --name directory-example
$ xvc pipeline --name directory-example step new --step-name directory-step --command "echo 'Directory has changed'"
$ xvc pipeline --name directory-example step dependency --step-name directory-step --directory dir-0001/
```

When you define the pipeline for the first time, it will run the step.

```console
$ xvc -vvvv pipeline --name directory-example run

```

If you run the pipeline again, it won't run the step because the directory hasn't changed.

```console
$ xvc pipeline --name directory-example run
```

If you add, delete or modify a file in the directory, the step will be invalidated and run again.

```console
$ touch dir-0001/another-file.txt
$ xvc pipeline --name directory-example run

```




## Caveats

## Tips

Most shells support editing longer commands with an editor. For bash, you can use `Ctrl+X Ctrl+E`.

Pipeline commands can get longer quickly. You can use [xvc aliases](/ref/xvc-aliases.md) for shorter
versions. Type `source $(xvc aliases)` to load the aliases into your shell.
