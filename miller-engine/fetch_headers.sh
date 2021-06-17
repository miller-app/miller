#!/bin/sh

# this script clones zengarden repo, removes unused stuff and generates
# wrapper.h.

root=$(pwd)/$(dirname "$0")
root=$(echo "$root" | sed "s/\/\.//")
zen_src="$root/src/cpp/headers"

cd "$root" || exit

clone_repo() {
    rm -rf "$zen_src"
    git clone https://github.com/miller-app/ZenGarden.git "$zen_src" 
}

mv_headers() {
    mv "$zen_src/src/"*.h "$zen_src"
}

rm_unused() {
    cd "$zen_src" || exit
    to_delete=$(find . -not \( -name '*.h' -or -name '.' -or -name '..' \))

    for item in $to_delete; do
        rm -rf "$item"
    done

    cd - || exit
}

clone_repo
mv_headers
rm_unused
