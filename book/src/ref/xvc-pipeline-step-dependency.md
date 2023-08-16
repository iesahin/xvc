# xvc pipeline step dependency

## Purpose

Define a dependency to an existing step in the pipeline.

## Synopsis

```console
$ xvc pipeline step dependency --help
Add a dependency to a step

Usage: xvc pipeline step dependency [OPTIONS] --step-name <STEP_NAME>

Options:
  -s, --step-name <STEP_NAME>
          Name of the step to add the dependency to

      --generic <GENERICS>
          Add a generic command output as a dependency. Can be used multiple times. Please delimit the command with ' ' to avoid shell expansion

      --url <URLS>
          Add a URL dependency to the step. Can be used multiple times

      --file <FILES>
          Add a file dependency to the step. Can be used multiple times

      --step <STEPS>
          Add a step dependency to a step. Can be used multiple times. Steps are referred with their names

      --glob <GLOBS>
          Add a glob dependency to the step. Can be used multiple times.

          The difference between this and the glob-digest option is that the glob option keeps track of all matching files, but glob-digest only keeps track of the matched files' digest. When you want to use ${[ALL_GLOB_FILES]} or ${[CHANGED_GLOB_FILES]} options in the step command, use the glob option. Otherwise, you can use the glob-digest option to save disk space.

      --glob_digest <GLOB_DIGESTS>
          Add a glob digest dependency to the step. Can be used multiple times.

          The difference between this and the glob option is that the glob option keeps track of all matching files, but glob-digest only keeps track of the matched files' digest. When you want to use ${[ALL_GLOB_FILES]} or ${[CHANGED_GLOB_FILES]} options in the step command, use the glob option. Otherwise, you can use the glob-digest option to save disk space.

      --param <PARAMS>
          Add a parameter dependency to the step in the form filename.yaml::model.units . Can be used multiple times

      --regex <REGEXPS>
          Add a regex dependency in the form filename.txt:/^regex/ . Can be used multiple times. The difference between this and the regex-digest option is that the regex option keeps track of all matching lines, but regex-digest only keeps track of the matched lines' digest. When you want to use ${[ALL_REGEX_LINES]} or ${[CHANGED_REGEX_LINES]} options in the step command, use the regex option. Otherwise, you can use the regex-digest option to save disk space

      --regex_digest <REGEXP_DIGESTS>
          Add a regex dependency in the form filename.txt:/^regex/ . Can be used multiple times.

          The difference between this and the regex option is that the regex option keeps track of all matching lines that can be used in the step command. This option only keeps track of the matched lines' digest.

      --line <LINES>
          Add a line dependency in the form filename.txt::123-234

          The difference between this and the line-digest option is that the line option keeps track of all matching lines that can be used in the step command. This option only keeps track of the matched lines' digest. When you want to use ${[ALL_LINES]} or ${[CHANGED_LINES]} options in the step command, use the line option. Otherwise, you can use the line-digest option to save disk space.

      --line_digest <LINE_DIGESTS>
          Add a line digest dependency in the form filename.txt::123-234

          The difference between this and the line option is that the line option keeps track of all matching lines that can be used in the step command. This option only keeps track of the matched lines' digest. When you want to use ${[ALL_LINES]} or ${[CHANGED_LINES]} options in the step command, use the line option. Otherwise, you can use the line-digest option to save disk space.

  -h, --help
          Print help (see a summary with '-h')

```

{{#include xvc-pipeline-step-dependency-file.md}}
{{#include xvc-pipeline-step-dependency-glob.md}}
{{#include xvc-pipeline-step-dependency-regex.md}}
{{#include xvc-pipeline-step-dependency-lines.md}}
{{#include xvc-pipeline-step-dependency-glob-items.md}}
{{#include xvc-pipeline-step-dependency-regex-items.md}}
{{#include xvc-pipeline-step-dependency-line-items.md}}
{{#include xvc-pipeline-step-dependency-param.md}}
{{#include xvc-pipeline-step-dependency-step.md}}
{{#include xvc-pipeline-step-dependency-url.md}}
{{#include xvc-pipeline-step-dependency-generic.md}}

## Caveats

## Tips

Most shells support editing longer commands with an editor. For bash, you can use `Ctrl+X Ctrl+E`.

Pipeline commands can get longer quickly. You can use [xvc aliases](/ref/xvc-aliases.md) for shorter
versions. Type `source $(xvc aliases)` to load the aliases into your shell.
