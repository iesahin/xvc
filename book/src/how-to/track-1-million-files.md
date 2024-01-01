# Track 1 Million Files

Machine learning pipelines consist of many files. A single pipeline can easily depend on and produce a milion files. XVC is designed to handle this scale of data.

Here, we'll experiment with the speed of handling such a data set. This is a stress test to see whether Xvc can handle this scale of data with acceptable performance. 

We'll first create 1 million files with 1 KB of random content. We'll run these in a repository without Git to test the capabilities without Git repositories. In general, it is better and more secure to work with Xvc in a Git repository. 

```console
$ xvc init --no-git
```

Let's create a million files in 100 directories. 

```console
$ zsh -cl 'for i in {1..100}; do mkdir -p dir-$i; for j in {1..10000} ; do dd if=/dev/urandom of=dir-$i/file-$i.bin bs=1024 count=1 ; done ; done'
```

Let's measure tracking performance:

```console
$ hyperfine -r 1 'xvc file track .'
```

