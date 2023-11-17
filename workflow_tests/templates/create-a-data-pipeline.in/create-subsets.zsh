#!/usr/bin/env zsh

for p in {1..60} ; do echo "Copying ${p}" ; xvc file copy --name-only 'data/data/input_${p}_*' data/train/ ; done

for p in {61..80} ; do echo "Copying ${p}" ; xvc file copy --name-only 'data/data/input_${p}_*' data/validate/ ; done

for p in {81..100} ; do echo "Copying ${p}" ; xvc file copy --name-only 'data/data/input_${p}_*' data/test/ ; done
