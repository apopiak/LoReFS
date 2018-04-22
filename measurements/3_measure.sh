#!/bin/bash

echo ::memstat | mdb -k

COUNTER=1
while [  $COUNTER -le 1000 ]; do
  modload /kernel/fs/amd64/lorefs
  mount -F lorefs /tmp/source /tmp/test
  echo "foo" >> /tmp/test/bar.txt
  umount /tmp/test
  modunload -i <module number>
  let COUNTER=COUNTER+1
done

echo ::memstat | mdb -k
