# xvc pipeline step remove

## Purpose

Remove a step and all its dependencies and outputs from the pipeline.

## Synopsis

```console
$ xvc pipeline step remove --help
Remove a step from a pipeline

Usage: xvc pipeline step remove --step-name <STEP_NAME>

Options:
  -s, --step-name <STEP_NAME>  Name of the step to remove
  -h, --help                   Print help

```

## Examples

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

Let's create a few steps and make them depend on each other.
```console
$ xvc pipeline step new --step-name hello --command 'echo hello >> hello.txt'

$ xvc pipeline step new --step-name world --command 'echo world >> world.txt'

$ xvc pipeline step new --step-name from --command 'echo from >> from.txt'

$ xvc pipeline step new --step-name xvc --command 'echo xvc >> xvc.txt'

```
Let's specify the outputs as well.
```console
$ xvc pipeline step output --step-name hello --output-file hello.txt

$ xvc pipeline step output --step-name world --output-file world.txt

$ xvc pipeline step output --step-name from --output-file from.txt

$ xvc pipeline step output --step-name xvc --output-file xvc.txt

```

Now we can add dependencies between them.
```console
$ xvc pipeline step dependency --step-name xvc --step from

$ xvc pipeline step dependency --step-name from --file world.txt

$ xvc pipeline step dependency --step-name world --step hello

```


Now the pipeline looks like this:
```console
$ xvc pipeline step list
hello: echo hello >> hello.txt (by_dependencies)
world: echo world >> world.txt (by_dependencies)
from: echo from >> from.txt (by_dependencies)
xvc: echo xvc >> xvc.txt (by_dependencies)

$ xvc pipeline dag --format mermaid
flowchart TD
    n0["hello"]
    n1["hello.txt"] --> n0
    n2["world"]
    n0["hello"] --> n2
    n3["world.txt"] --> n2
    n4["from"]
    n3["world.txt"] --> n4
    n5["from.txt"] --> n4
    n6["xvc"]
    n4["from"] --> n6
    n7["xvc.txt"] --> n6


```

When we remove a step, all its dependencies and outputs are removed as well.
```console
$ xvc -vv pipeline step remove --step-name from
[INFO] Removing dep: file(world.txt)
[INFO] Removing step dep step(from) from xvc
[INFO] Removing output: File

```

```console
$ xvc pipeline step list
hello: echo hello >> hello.txt (by_dependencies)
world: echo world >> world.txt (by_dependencies)
xvc: echo xvc >> xvc.txt (by_dependencies)

$ xvc pipeline dag --format mermaid
flowchart TD
    n0["hello"]
    n1["hello.txt"] --> n0
    n2["world"]
    n0["hello"] --> n2
    n3["world.txt"] --> n2
    n4["xvc"]
    n5["xvc.txt"] --> n4


```

