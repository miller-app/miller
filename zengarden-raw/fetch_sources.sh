#!/bin/sh

# this script clones zengarden repo, removes unused stuff and generates
# wrapper.h.

root=$(pwd)/$(dirname "$0")
root=$(echo "$root" | sed "s/\/\.//")
zen_src="$root/zengarden"

cd "$root" || exit

clone_repo() {
    rm -rf "$zen_src"
    git clone https://github.com/miller-app/ZenGarden.git "$zen_src" 
}

rm_unused() {
    cd "$zen_src" || exit
    rm -rf \
        .git \
        .gitignore \
        OBJECTS.txt \
        Xcode \
        _clang-format \
        examples \
        jni \
        junit-4.8.2.jar \
        libs \
        pyExampleGarden.py \
        runme-java.sh \
        runme-python.sh \
        scripts \
        src/ZenGardenDS \
        src/main.cpp \
        src/me
    cd - || exit
}

generate_wrapper() {
    wrapper="$root/wrapper.hpp"
    rm -f "$wrapper"
    touch "$wrapper"

    {
        echo '// this file is automatically generated by command:'
        echo '// ./fetch_sources.sh'
        echo '#include "ZGCallbackFunction.h"'
        echo '#include "ZenGarden.h"'
    } >> "$wrapper"
}

clone_repo
rm_unused
generate_wrapper
