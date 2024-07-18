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

      --glob_items <GLOB_ITEMS>
          Add a glob items dependency to the step.

          You can depend on multiple files and directories with this dependency.

          The difference between this and the glob option is that this option keeps track of all matching files, but glob only keeps track of the matched files' digest. When you want to use ${XVC_GLOB_ITEMS}, ${XVC_ADDED_GLOB_ITEMS}, or ${XVC_REMOVED_GLOB_ITEMS} environment variables in the step command, use the glob-items dependency. Otherwise, you can use the glob option to save disk space.

      --glob <GLOBS>
          Add a glob dependency to the step. Can be used multiple times.

          You can depend on multiple files and directories with this dependency.

          The difference between this and the glob-items option is that the glob-items option keeps track of all matching files individually, but this option only keeps track of the matched files' digest. This dependency uses considerably less disk space.

      --param <PARAMS>
          Add a parameter dependency to the step in the form filename.yaml::model.units . Can be used multiple times

      --regex_items <REGEX_ITEMS>
          Add a regex dependency in the form filename.txt:/^regex/ . Can be used multiple times.

          The difference between this and the regex option is that the regex-items option keeps track of all matching lines, but regex only keeps track of the matched lines' digest. When you want to use ${XVC_REGEX_ITEMS}, ${XVC_ADDED_REGEX_ITEMS}, ${XVC_REMOVED_REGEX_ITEMS} environment variables in the step command, use the regex option. Otherwise, you can use the regex-digest option to save disk space.

      --regex <REGEXES>
          Add a regex dependency in the form filename.txt:/^regex/ . Can be used multiple times.

          The difference between this and the regex option is that the regex option keeps track of all matching lines that can be used in the step command. This option only keeps track of the matched lines' digest.

      --line_items <LINE_ITEMS>
          Add a line dependency in the form filename.txt::123-234

          The difference between this and the lines option is that the line-items option keeps track of all matching lines that can be used in the step command. This option only keeps track of the matched lines' digest. When you want to use ${XVC_ALL_LINE_ITEMS}, ${XVC_ADDED_LINE_ITEMS}, ${XVC_CHANGED_LINE_ITEMS} options in the step command, use the line option. Otherwise, you can use the lines option to save disk space.

      --lines <LINES>
          Add a line digest dependency in the form filename.txt::123-234

          The difference between this and the line-items dependency is that the line option keeps track of all matching lines that can be used in the step command. This option only keeps track of the matched lines' digest. If you don't need individual lines to be kept, use this option to save space.

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
