#!/bin/bash

set -vuex

curl -L https://github.com/badboy/mdbook-mermaid/releases/download/v0.12.3/mdbook-mermaid-v0.12.3-x86_64-unknown-linux-gnu.tar.gz | tar xvz
curl -L https://github.com/rust-lang/mdBook/releases/download/v0.4.19/mdbook-v0.4.19-x86_64-unknown-linux-gnu.tar.gz | tar xvz
PATH=${PATH}:${PWD} mdbook build
