### Regex Dependencies

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

Now, let's add a step to the pipeline to count females in the file:

```console
$ xvc pipeline step new --step-name count-females --command "grep -c '\"F\",' people.csv"
```

These commands are run when the regex dependencies change.

```console
$ xvc pipeline step dependency --step-name count-females --regex 'people.csv:/^.*"F",.*$'

```

When you run the pipeline initially, the steps are run.

```console
$ xvc pipeline run
[OUT] [count-females] 7

[DONE] [count-females] (grep -c '"F",' people.csv)


``````

When you run the pipeline again, the step is not run because the regex result didn't change.

```console
$ xvc pipeline run

``````

When you add a new female record to the file, the step is run and the command prints the new count.

```console
$ zsh -c "echo '\"Asude\",      \"F\",   12,       55,      110' >> people.csv"

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

"Asude",      "F",   12,       55,      110

$ xvc pipeline run
[OUT] [count-females] 8

[DONE] [count-females] (grep -c '"F",' people.csv)


```
