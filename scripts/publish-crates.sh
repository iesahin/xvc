#!/bin/zsh

# Adding -e to this mix causes script to fail when cargo publish finds an already published crate.
set -vux


for dir in test_helper logging ecs walker config core pipeline storage file lib workflow_tests ; do
    cd "$dir"
    cargo publish
    cd -
done

