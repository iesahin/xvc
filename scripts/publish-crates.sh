#!/bin/bash

# Adding -e to this mix causes script to fail when cargo publish finds an already published crate.
set -vux

local base="$(dirname $0)/.."

for dir in test_helper logging ecs walker config core pipeline storage file lib workflow_tests ; do
    cargo update
    cd "${base}/${dir}"
    cargo publish
    cd -
done

