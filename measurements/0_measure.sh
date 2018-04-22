#!/bin/bash

echo ::memstat | mdb -k

modload /kernel/fs/amd64/lorefs
modunload -i <module number>

echo ::memstat | mdb -k
