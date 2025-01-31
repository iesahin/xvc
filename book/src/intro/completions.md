# Adding completions to your shell

Xvc supports dynamic completions for bash, zsh, elvish, fish and powershell. 

This means, when you hit `TAB` in your shell, it calls Xvc to complete the
command. Even paths that are not visible in your filesystem or pipeline and
step names are completed this way. 

In order to activate completions, run the following commands once in your shell:

## Bash

```bash
echo "source <(COMPLETE=bash xvc)" >> ~/.bashrc
```

## Elvish

```sh
echo "eval (E:COMPLETE=elvish xvc | slurp)" >> ~/.elvish/rc.elv
```

## Fish

```sh
echo "source (COMPLETE=fish xvc | psub)" >> ~/.config/fish/config.fish
```

## Powershell

```sh
$env:COMPLETE = "powershell"
echo "xvc | Out-String | Invoke-Expression" >> $PROFILE
Remove-Item Env:\COMPLETE
```

## Zsh

```sh
echo "source <(COMPLETE=zsh xvc)" >> ~/.zshrc
```

## Nushell (without dynamic completions)

Until [`clap_complete_nushell`](https://lib.rs/crates/clap_complete_nushell) supports dynamic completions, similar to the above, you can create a completion script with `xvc` and use it on your shell. 

```nu
$ xvc _comp generate-nushell | save ($nu.config-path | path dirname | path join "xvc-completions.nu")
$ use ($nu.config-path | path dirname | path join "xvc-completions.nu") *
```

This will provide completions for commands and options. It won't work for dynamic completions like pipelines names, storage id's etc. 


