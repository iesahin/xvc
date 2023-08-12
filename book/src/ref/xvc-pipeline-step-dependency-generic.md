### Generic Command Dependencies

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

You can use the output of a shell command as a dependency to a step.
When the command is run, the output hash is saved to compare and the step is invalidated when the output of the command changed.

You can use this for any command that outputs a string.

```console
$ xvc pipeline step new --step-name morning-message --command "echo 'Good Morning!'"

$ xvc  pipeline step dependency --step-name morning-message --generic 'date +%F'

```

The step is invalidated when the date changes and the step is run again.

```console
$ xvc pipeline run
[OUT] [morning-message] Good Morning!


```

The step won't run until tomorrow, when `date +%F` changes.

```console
$ xvc pipeline run

```

You can mimic all kinds of pipeline behavior with this generic dependency.

For example, if you want to run a command when directory contents change, you can depend on the output of `ls -lR`:

```console
$ xvc pipeline step new --step-name directory-contents --command "echo 'Files changed'"
$ xvc pipeline step dependency --step-name directory-contents --generic 'ls -lR'
$ xvc pipeline run
[OUT] [directory-contents] Files changed
```

When you add a file to the directory, the step is invalidated and run again:

```console
$ xvc pipeline run
$ touch new-file
$ xvc pipeline run
[OUT] [directory-contents] Files changed
```
