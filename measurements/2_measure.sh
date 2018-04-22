#!/bin/bash

echo ::memstat | mdb -k

COUNTER=0
while [  $COUNTER -le 1000 ]; do
  modload /kernel/fs/amd64/lorefs
  modunload -i <module number>
  let COUNTER=COUNTER+1
done

echo ::memstat | mdb -k
