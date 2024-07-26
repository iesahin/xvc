### SQLite Query Dependency

You can create a step dependency with an SQLite query. When the query results
change, the step is invalidated.

SQLite dependencies doesn't track the results of the query. It just checks
whether the query results has changed.

This command works only in Xvc repositories.

```console
$ git init
...
$ xvc init
```

Let's create an SQLite database and populate a table:

```console
$ sqlite3 people.db <<EOF

-- Create the table
CREATE TABLE People (
    Name TEXT,
    Sex TEXT,
    Age INTEGER,
    Height_in INTEGER,
    Weight_lbs INTEGER
);

-- Insert the data
INSERT INTO People (Name, Sex, Age, Height_in, Weight_lbs) VALUES
('Alex', 'M', 41, 74, 170),
('Bert', 'M', 42, 68, 166),
('Carl', 'M', 32, 70, 155),
('Dave', 'M', 39, 72, 167),
('Elly', 'F', 30, 66, 124),
('Fran', 'F', 33, 66, 115),
('Gwen', 'F', 26, 64, 121),
('Hank', 'M', 30, 71, 158),
('Ivan', 'M', 53, 72, 175),
('Jake', 'M', 32, 69, 143),
('Kate', 'F', 47, 69, 139),
('Luke', 'M', 34, 72, 163),
('Myra', 'F', 23, 62, 98),
('Neil', 'M', 36, 75, 160),
('Omar', 'M', 38, 70, 145),
('Page', 'F', 31, 67, 135),
('Quin', 'M', 29, 71, 176),
('Ruth', 'F', 28, 65, 131);
EOF

```

Now, we'll add a step to the pipeline to calculate the average age of these people.

```console
$ xvc pipeline step new --step-name average-age --command 'sqlite3 people.db "SELECT AVG(Age) FROM People;"'
[DEBUG][logging/src/lib.rs::237] Terminal logger enabled with level: Debug
[DEBUG][config/src/error.rs::72] Config source for level "system" not found at "/Users/iex/Library/Application Support/com.emresult.xvc"
[DEBUG][config/src/error.rs::72] Config source for level "global" not found at "/Users/iex/Library/Application Support/xvc"
[DEBUG] Using Git: /opt/homebrew/bin/git
[DEBUG] Committing .xvc/ to git: [main d67183e] Xvc auto-commit after '/Users/iex/github.com/iesahin/xvc/target/debug/xvc -vvv pipeline step new --step-name average-age --command sqlite3 people.db "SELECT AVG(Age) FROM People;"'
 6 files changed, 6 insertions(+)
 create mode 100644 .xvc/ec/1722021403030243
 create mode 100644 .xvc/store/xvc-step-command-store/1722021403029764.json
 create mode 100644 .xvc/store/xvc-step-invalidate-store/1722021403029598.json
 create mode 100644 .xvc/store/xvc-step-store/1722021403029432.json
 create mode 100644 .xvc/store/xvc-step-store/1722021403030038.json
 create mode 100644 .xvc/store/xvc-step-xvc-pipeline-r1n-store/1722021403030134.json

[DEBUG] Command completed successfully.

```

Let's run the step without a dependency first.

```console
$ xvc -vvv pipeline run
[ERROR] Step average-age finished UNSUCCESSFULLY with command sqlite3 people.db "SELECT AVG(Age) FROM People;"

```

Now, we'll add a dependency to this step and it will only run the step when the results of that query changes.

```console
$ xvc pipeline step dependency --step-name average-age --sqlite-query people.db 'SELECT count(*) FROM People;'

```

```note
The dependency query is run everytime the pipeline runs. It's expected to be lightweight to avoid performance issues.
```

So, when the number of people in the table changes, the step will run. Initially it doesn't keep track of the query results, so it will run again.

```console
$ xvc pipeline run
thread '<unnamed>' panicked at pipeline/src/pipeline/deps/compare.rs:474:50:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[ERROR] Error in step thread: Any { .. }

```

But it won't run the step a second time, as the table didn't change.

```console
$ xvc pipeline run
thread '<unnamed>' panicked at pipeline/src/pipeline/deps/compare.rs:474:50:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[ERROR] Error in step thread: Any { .. }

```

Let's add another row to the table:

```console
$ sqlite3 people.db "INSERT INTO People (Name, Sex, Age, Height_in, Weight_lbs) VALUES ('Asude', 'F', 10, 74, 170);"
```

This time, the step will run again as the result from dependency query (`SELECT count(*) FROM People`) changed.

```console
$ xvc pipeline run
thread '<unnamed>' panicked at pipeline/src/pipeline/deps/compare.rs:474:50:
not yet implemented
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[ERROR] Error in step thread: Any { .. }

```

```note
Xvc opens the database in read-only mode to avoid locking.
```
