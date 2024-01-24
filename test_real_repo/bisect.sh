#!/bin/bash

# Bisect the generated repo

if [ "$#" -ne 1 ]; then
    echo "Illegal number of parameters. Usage: bisect.sh <first bad counter>"
    exit 1
fi

cutoff=$1
counter=$(echo counter.txt)

if [ "$(cat counter.txt)" -lt $cutoff ]; then
    echo "Number in the file is less than $cutoff. Exiting with status 0."
    exit 0
else
    echo "Number in the file is not less than $cutoff. Exiting with status 1."
    exit 1
fi
