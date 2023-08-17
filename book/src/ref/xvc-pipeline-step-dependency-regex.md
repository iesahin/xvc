### Regex Digest Dependencies

You can specify a regular expression matched against the lines from a file as a dependency. The step is invalidated when
the matched results changed.

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

We'll use a sample CSV file in this example:

```console
$ cat people.csv
```

Now, let's add steps to the pipeline to count males and females in the file:

```console
$ xvc pipeline step new --step-name count-males --command "grep -c '"M",' people.csv"
$ xvc pipeline step new --step-name count-females --command "grep -c '"F",' people.csv"
```

These commands must be run when the respective regexes changed.

```console
$ xvc pipeline step dependency --to count-males --regex '"M",'
$ xvc pipeline step dependency --to count-females --regex '"F",'
```

When you run the pipeline initially, the steps are run.

```console
$ xvc pipeline run
``````

When you run the pipeline again, the steps are not run because the regexes didn't change.

```console
$ xvc pipeline run
``````

When you add a new female record to the file, only the female count step is run.

```console
$ echo '"Asude",       "F",   12,       55,      110' >> people.csv
$ xvc pipeline run
```
