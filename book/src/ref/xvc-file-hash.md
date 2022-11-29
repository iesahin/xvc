# xvc file hash


## Synopsis 

```console
$ xvc file hash --help
Get digest hash of files with the supported algorithms

Usage: xvc file hash [OPTIONS] [TARGETS]...

Arguments:
  [TARGETS]...  Files to process

Options:
  -a, --algorithm <ALGORITHM>
          Algorithm to calculate the hash. One of blake3, blake2, sha2, sha3. All algorithm variants produce 32-bytes digest
      --text-or-binary <TEXT_OR_BINARY>
          For "text" remove line endings before calculating the digest. Keep line endings if "binary". "auto" (default) detects the type by checking 0s in the first 8Kbytes, similar to Git [default: auto]
  -h, --help
          Print help information
  -V, --version
          Print version information

```
