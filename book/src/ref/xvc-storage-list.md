# xvc storage list

## Purpose

List all configured storages with their names and guids. 

## Synopsis 

```text
{{#include xvc-storage-list.txt}}
```

## Examples

List all storages in the repository:

```shell
$ xvc storage list 
```

## Caveats

This one uses the local configuration and doesn't try to connect storages.
If it's listed with the command, it doesn't mean it's guaranteed to be able to pull or push. 


