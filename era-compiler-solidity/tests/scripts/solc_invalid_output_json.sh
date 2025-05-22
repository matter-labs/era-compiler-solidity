#!/bin/bash

if [ "$1" == "--version" ]; then
    echo 'solc, the solidity compiler commandline interface'
    echo 'Version: 0.8.29+commit.deadbeef.platform.toolchain'
    echo 'ZKsync: 0.8.29-1.0.2'
    exit 0
fi

input="$(</dev/stdin)"

echo 'Invalid JSON output'
