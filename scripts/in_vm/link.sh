#!/bin/bash

# go to the compilation directory for lorefs and link the rust lib into the binary
pushd /export/home/schillix/schillix-on/usr/src/uts/intel/lorefs

/usr/ccs/bin/ld -r -o obj64/lorefs obj64/lorefs_subr.o obj64/lorefs_vfsops.o obj64/lorefs_vnops.o -L ./obj64 -l lorefs

popd
