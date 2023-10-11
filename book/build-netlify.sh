#!/bin/bash

set -vuex

curl -L https://github.com/tommilligan/mdbook-admonish/releases/download/v1.13.0/mdbook-admonish-v1.13.0-x86_64-unknown-linux-gnu.tar.gz | tar xvz
curl -L https://github.com/badboy/mdbook-mermaid/releases/download/v0.12.6/mdbook-mermaid-v0.12.6-x86_64-unknown-linux-gnu.tar.gz | tar xvz
curl -L https://github.com/rust-lang/mdBook/releases/download/v0.4.35/mdbook-v0.4.35-x86_64-unknown-linux-gnu.tar.gz | tar xvz

PATH=${PATH}:${PWD} mdbook build
