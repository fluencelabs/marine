#!/usr/bin/env bash

set -euo pipefail

for dir in ./*; do
    # skip non-directory entries
    [ -d "$dir" ] || continue

    # skip if there's no build.sh in the directory
    [ -e "$dir/build.sh" ] || continue

    (cd "$dir"; ./build.sh)
done
