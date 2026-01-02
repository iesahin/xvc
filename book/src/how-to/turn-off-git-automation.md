# Turning off Automated Git Operations

By default Xvc automates all common git operations. When you run an Xvc operation that affects the files under `.xvc` directory, the changes are committed to the repository automatically. 

Git autmation runs in Git repositories. 

```console
$ git init
Initialized empty Git repository in [CWD]/.git/

$ xvc init
```

We'll show these examples in the following directory tree. 

```console
$ xvc-test-helper create-directory-tree --directories 1 --files 3 --seed 20231012
$ tree
.
└── dir-0001
    ├── file-0001.bin
    ├── file-0002.bin
    └── file-0003.bin

2 directories, 3 files

```

When you begin to track a file in the repository, Xvc adds the file to .gitignore in the directory the file is found. 

```console
$ xvc file track dir-0001/file-0001.bin

$ zsh -cl 'cat dir-0001/.gitignore'
### Following 1 lines are added by xvc on [..]
/file-0001.bin

```

Xvc also adds a commit for all the changes caused by the command. 

```console
$ git log -n 1
commit [..]
Author: [..]
Date:   [..]

    Xvc auto-commit after '[..]xvc file track dir-0001/file-0001.bin'

```

The commit message includes the command you gave to run to find the exact change in history. 

If you don't track a file with Xvc, they are not added to `.gitignore` and you can see them with `git status`. 

```console
$ git status -s
?? dir-0001/file-0002.bin
?? dir-0001/file-0003.bin

```
If you want to skip this automated Git operations, you can add `--skip-git` flag to commands. 

```console
$ xvc --skip-git file track dir-0001/file-0002.bin

$ git status -s
 M dir-0001/.gitignore
?? .xvc/ec/[..]
?? .xvc/store/[..]
?? .xvc/store/[..]
?? .xvc/store/[..]
?? .xvc/store/[..]
?? .xvc/store/[..]
?? dir-0001/file-0003.bin

```

Note that, `--skip-git` flag doesn't affect the files to be added to `.gitignore` files. 

```console
$ zsh -cl 'cat dir-0001/.gitignore'
### Following 1 lines are added by xvc on [..]
/file-0001.bin
### Following 1 lines are added by xvc on [..]
/file-0002.bin

```

You can use usual Git workflow to add and commit the files.

```
$ git add .xvc dir-0001/.gitignore
$ git commit -m "Began to track dir-0001/file-0002.bin with Xvc"
[main [..]] Began to track dir-0001/file-0002.bin with Xvc
 7 files changed, 8 insertions(+)
 create mode 100644 .xvc/ec/[..]
 create mode 100644 .xvc/store/[..].json
 create mode 100644 .xvc/store/[..].json
 create mode 100644 .xvc/store/[..].json
 create mode 100644 .xvc/store/[..].json
 create mode 100644 .xvc/store/[..].json

```

If you never want Xvc to handle commits, you can set `git.use_git` option in
`.xvc/config.toml` file to false or set `XVC_git.use_git=false` in the environment.

```console
$ XVC_git.use_git=false xvc file track dir-0001/file-0003.bin
# Or XVC_GIT.USE_GIT=false
```
