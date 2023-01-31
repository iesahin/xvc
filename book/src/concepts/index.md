# Concepts

- [Digest](./digest.md): A digest is a 32-byte numeric sequence to identify a file, content or any other data. Xvc uses different algorithms to generate this sequence.
- [Associated Digest](./associated-digest.md): This is a specific kind of digest associated with an entity. An entity can have more than one digests, like *content digest* or *metadata digest*. Xvc uses these different kinds of digests to avoid unnecessary digest calculations.
- [Recheck](./recheck.md): Recheck is the process of linking a file to its copy in Xvc cache. Xvc uses different methods to recheck a file, like *copy*, *symlink*, *hardlink* or *reflink*.
- [Workspace](./workspace.md): A project is broadly divided into 3 different types of directories. `.xvc/` contains the
  cache and metadata of the tracked files and pipelines, `.git/` contains the git repository and the workspace contains
  the files that are tracked by either Xvc or git. It's the place where you do your work.
- [Carry-In](./carry-in.md): Carry-in is the process of adding a new version of a file to Xvc cache. It's analogous to
  `git commit`.
