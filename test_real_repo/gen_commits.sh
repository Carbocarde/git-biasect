#!/bin/bash

# Generate a git repo with a counter.txt file incremented every commit.

if [ "$#" -ne 1 ]; then
    echo "Illegal number of parameters. Usage: gen_commits.sh '<repo path>'"
    exit 1
fi

repo=$1

for ((i=0; i<=80000; i++)); do
    echo $i > $repo/counter.txt
    git -C $repo add counter.txt
    git -C $repo commit -m "$i"
done
