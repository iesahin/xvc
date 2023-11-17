#!/usr/bin/env zsh

for p in {1..60} ; do xvc file copy 'data/data/input_${p}_*' data/train/ ; done

for p in {61..80} ; do xvc file copy 'data/data/input_${p}_*' data/validate/ ; done

for p in {81..100} ; do xvc file copy 'data/data/input_${p}_*' data/test/ ; done
