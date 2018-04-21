#!/bin/bash
set -ex

# copy the header
scp -P 2022 \
    lorefs_c/lorefs_rust.h \
    schillix@localhost:/export/home/schillix/schillix-on/usr/src/uts/common/fs/lorefs/

# copy the library
scp -P 2022 \
    target/x86_64-sun-solaris/release/liblorefs.a \
    schillix@localhost:/export/home/schillix/schillix-on/usr/src/uts/intel/lorefs/obj64/
