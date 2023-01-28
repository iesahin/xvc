# workspace

A project is broadly divided into 3 different directories:
- `.git/` contains the files associated with Git.
- `.xvc/` contains the cache and metadata of the tracked files and pipelines
- The *workspace* contains the files that are tracked by either Xvc or git (or ignored by them.)

We use the term generally to describe the files the user works with, and in contrast to Xvc cache files under `.xvc/`.
