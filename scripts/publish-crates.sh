#!/bin/zsh

set -vuex

for dir in test_helper logging ecs walker config core pipeline storage file lib workflow_tests ; do
    cd ${dir}
    cargo publish
    cd -
done

