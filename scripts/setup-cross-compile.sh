#!/bin/bash
set -ex

# cross compilation setup in case of using `cross_compile.sh` instead of `cross`

# install the toolchain for the cross compiling docker container
rustup toolchain install nightly-x86_64-unknown-linux-gnu
# add the cross compilation target (x86_64-sun-solaris)
rustup target add --toolchain nightly-x86_64-unknown-linux-gnu x86_64-sun-solaris
rustup component add --toolchain nightly-x86_64-unknown-linux-gnu rust-src
