#!/usr/bin/env sh

set -v

for file in `find . -type f `; do
    mimetype=$(tika --json $file | jq '.["Content-Type"]' | tr -d "\"")

    echo $mimetype
    mkdir -p "./$mimetype"

    mv "$file" "$mimetype/"
done
