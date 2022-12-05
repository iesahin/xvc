# Xvc with Git Branches

When you're working with multiple branches in Git, you may ask Xvc to checkout a branch and commit to another branch. 
These operations are performed at the beginning, and at the end of Xvc operations. 
You can use `--from-ref` and `--to-branch` options to checkout a Git reference before an Xvc operation, and commit the results to a certain Git branch.

Checkout and commit operations sandwich Xvc operations. 

```mermaid
graph LR
   checkout["git checkout $REF"] --> xvc
   xvc["xvc operation"] --> stash["git stash --staged"]
   stash --> branch["git checkout --branch $TO_BRANCH"]
   branch --> commit["git add .xvc && git commit"]
```

If `--from-ref` is not given, initial `git checkout` is not performed. 
Xvc operates in the current branch. 
This is the default behavior. 

```console
$ git init --initial-branch=main
...
$ xvc init
? 0
$ ls
data.txt

$ xvc --to-branch data-file file track data.txt
Switched to a new branch 'data-file'

$ git branch
* data-file
  main

$ git status -s
$ xvc file list data.txt
C=	...                                         19	data.txt	        
```

If you return to `main` branch, you'll see the file is tracked by neither Git nor Xvc. 

```console
$ git checkout main
$ xvc file list
UU        data.txt
$ git status -s
?? data.txt
```

Now, we'll add a step to the default pipeline to get an uppercase version of the data. 
We want this to work only in data 

```console
$ xvc --from-ref data-file --to-branch data-file pipeline step new --step-name to-uppercase --command 'cat data.txt | tr a-z A-Z > uppercase.txt'
$ xvc pipeline step dependency --step-name to-uppercase --file data.txt pipeline
$ xvc pipeline step output --step-name to-uppercase --output-file uppercase.txt
```

Note that `xvc pipeline step dependency` and `xvc pipeline step output` commands don't need `--from-ref` and `--to-branch` options, as they run in `data-file` branch already. 

Now, we want to have this new version of data available only in `uppercase` branch. 

```console
$ xvc --from-ref data-file --to-branch uppercase pipeline run
Switched to branch 'data-file'
Switched to a new branch 'uppercase'
$ git branch
  data-file
  main
* uppercase

```

You can use this for experimentation. 
Whenever you have a pipeline that you want to run and keep the results in another Git branch, you can use `--to-branch` for experimentation. 

```console
$ xvcpr --from-ref data-file --to-branch another-uppercase
$ git-branch 
* another-uppercase
uppercase
data-file
main
```

The pipeline always runs, because in `data-file` branch `uppercase.txt` is always missing. 
It's stored only in the resulting branch you give by `--to-branch`. 




