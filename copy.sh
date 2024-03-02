#!/bin/bash
# copy over all files in directory to Meerkat directory ~/tel-sw/[your user name]
rsync -r --exclude='/.git' --filter="dir-merge,- .gitignore" ./ yjsp@tel-testbench.netbird.cloud:~/tel-sw/$USER