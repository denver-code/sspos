#!/bin/sh

img="disk.img"
path="/tmp/sspos"

# pip install fusepy
mkdir -p $path
echo "Mounting $img in $path"
python run/sspos-fuse.py $img $path
