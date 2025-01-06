# xvc completions

## Synopsis

```console
$ xvc completions --help
```

## Examples

Xvc can print out shell completions script for bash, elvish, fish, powershell and zsh. You can save these scripts to the completion script directory, e.g., `~/.local/share/bash-completion/completions/` or `/usr/share/bash-completion/completions/` for bash and let the shell load these at start.

You can also use sourcing in .bashrc or .zshrc, like:

```sh
source <(xvc completions)
```
and let the shell load completions at init. Please note `<` character at start, basically it will save the output to a temporary file and source it.

For fish, you can use a similar approach:

```sh
source (xvc completions fish | psub)
```

For PowerShell and Elvish please refer to their documentation. 
