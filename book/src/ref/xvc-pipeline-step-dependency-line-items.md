### Line Item Dependencies

You can make your steps to depend on lines of text files. The lines are defined by starting and ending indices.

When the text in those lines change, the step is invalidated.

Unlike line dependencies, this dependency type keeps track of the lines in the
file. You can use `${XVC_ALL_LINE_ITEMS}`, `${XVC_ADDED_LINE_ITEMS}`, and
`${XVC_REMOVED_LINE_ITEMS}` environment variables in the command. Please be
aware that for large set of lines, this dependency can take up considerable
space to keep track of all lines and if you don't need to keep track of changed
lines, you can use `--lines` dependency.

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

We'll use a sample CSV file in this example:

```console
$ cat people.csv
"Name",     "Sex", "Age", "Height (in)", "Weight (lbs)"
"Alex",       "M",   41,       74,      170
"Bert",       "M",   42,       68,      166
"Carl",       "M",   32,       70,      155
"Dave",       "M",   39,       72,      167
"Elly",       "F",   30,       66,      124
"Fran",       "F",   33,       66,      115
"Gwen",       "F",   26,       64,      121
"Hank",       "M",   30,       71,      158
"Ivan",       "M",   53,       72,      175
"Jake",       "M",   32,       69,      143
"Kate",       "F",   47,       69,      139
"Luke",       "M",   34,       72,      163
"Myra",       "F",   23,       62,       98
"Neil",       "M",   36,       75,      160
"Omar",       "M",   38,       70,      145
"Page",       "F",   31,       67,      135
"Quin",       "M",   29,       71,      176
"Ruth",       "F",   28,       65,      131


```

Let's a step to show the first 10 lines of the file:

```console
$ xvc pipeline step new --step-name print-top-10 --command 'echo "Added Lines:\n ${XVC_ADDED_LINE_ITEMS}\nRemoved Lines:\n${XVC_REMOVED_LINE_ITEMS}"'

```

The command is run only when those lines change.

```console
$ xvc pipeline step dependency --step-name print-top-10 --line-items 'people.csv::1-10'

```

When you run the pipeline initially, the step is run.

```console
$ xvc pipeline run
[OUT] [print-top-10] Added Lines:
 "Alex",       "M",   41,       74,      170
"Bert",       "M",   42,       68,      166
"Carl",       "M",   32,       70,      155
"Dave",       "M",   39,       72,      167
"Elly",       "F",   30,       66,      124
"Fran",       "F",   33,       66,      115
"Gwen",       "F",   26,       64,      121
"Hank",       "M",   30,       71,      158
"Ivan",       "M",   53,       72,      175
Removed Lines:

 
[DONE] print-top-10 (echo "Added Lines:/n ${XVC_ADDED_LINE_ITEMS}/nRemoved Lines:/n${XVC_REMOVED_LINE_ITEMS}")

``````

When you run the pipeline again, the step is not run because the specified lines didn't change.

```console
$ xvc pipeline run

``````

When you change a line from the file, the step is invalidated.

```console
$ perl -i -pe 's/Hank/Ferzan/g' people.csv

```

Now, when you run the pipeline, it will print the changed line, with its new and old versions.

```
$ xvc pipeline run
[OUT] [print-top-10] Added Lines:
 "Ferzan",       "M",   30,       71,      158
Removed Lines:
"Hank",       "M",   30,       71,      158
 
[DONE] print-top-10 (echo "Added Lines:/n ${XVC_ADDED_LINE_ITEMS}/nRemoved Lines:/n${XVC_REMOVED_LINE_ITEMS}")

```

