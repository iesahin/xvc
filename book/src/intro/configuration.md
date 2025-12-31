# Configuration

Xvc uses a hierarchical configuration system that allows you to set options at different levels. This provides flexibility, allowing for global settings, per-project settings, and on-the-fly overrides.

## Configuration Layers

Xvc merges configuration from multiple sources in a specific order of precedence. Settings from sources later in the list will override those from earlier ones.

1.  **Default Values**: Sensible defaults are hard-coded into Xvc.
2.  **System-wide Configuration**: For settings that apply to all users on a system (e.g., in `/etc/xvc/config.toml` on Linux).
3.  **User-specific Configuration**: For your personal preferences across all your projects (e.g., `~/.config/xvc/config.toml`).
4.  **Project Configuration**: This is the main configuration file for a project, located at `.xvc/config.toml`. It should be checked into Git to be shared with all collaborators.
5.  **Local Project Configuration**: For project-specific overrides that you don't want to share, such as local paths or credentials. This file is located at `.xvc/config.local.toml` and should be in your `.gitignore`.
6.  **Environment Variables**: You can override any configuration setting using environment variables.
7.  **Command-line Arguments**: The highest precedence, for temporary changes for a single command execution.

## Configuration Files

Configuration files are written in the [TOML](https://toml.io) format. They consist of sections (like `[core]` or `[git]`) and key-value pairs.

### Example `.xvc/config.toml`

```toml
[core]
# Default verbosity level. One of "error", "warn", "info", "debug", "trace"
verbosity = "info"

[git]
# Set to false if you want to manage git operations manually.
use_git = true
auto_commit = true

[file.track]
# By default, Xvc doesn't track files that are already tracked by Git.
# Set this to true to override that behavior.
include_git_files = false
```

You can generate a fully commented default configuration file by running `xvc init` and inspecting the generated `.xvc/config.toml`.

## Configure with Environment Variables

You can override any configuration setting by setting an environment variable. The variable name is constructed by:
1.  Starting with the `XVC_` prefix.
2.  Appending the section and key, separated by dots (e.g., `core.verbosity`). The lookup is case-insensitive.

For example, to change the `verbosity` in the `core` section, you would set:

```shell
export XVC_core.verbosity=debug
# Or:
export XVC_CORE.VERBOSITY=debug
```

To set `include_git_files` in the `[file.track]` section:

```shell
export XVC_file.track.include_git_files=true
# Or:
export XVC_FILE.TRACK.INCLUDE_GIT_FILES=true
```

## Changing configuration for a command

You can temporarily override configuration values for a single command using the `-c` or `--config` flag. The format is `section.key=value` (e.g., `git.use_git=false`). The lookup is case-insensitive.

```shell
$ xvc -c git.use_git=false file track my-data/
# Or:
$ xvc -c Git.Use_Git=false file track my-data/
```

This will disable Git automation just for this `xvc file track` command.

You can also disable entire configuration sources for a single command:
- `--no-system-config`: Ignores the system-wide configuration file.
- `--no-user-config`: Ignores your user-specific configuration file.
- `--no-project-config`: Ignores the project's `.xvc/config.toml`.
- `--no-local-config`: Ignores the local `.xvc/config.local.toml`.
- `--no-env-config`: Ignores all `XVC_` environment variables.
