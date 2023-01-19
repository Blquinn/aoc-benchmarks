#!/bin/bash

set -e

export CC=clang
export CXX=clang++

mkdir -p cpp/build
cd cpp/build
meson .. &>/dev/null
meson configure -Dbuildtype=release -Ddebug=false -Doptimization=3 &>/dev/null

ninja &>/dev/null
echo "Running benchmark."
./bench

set +e

cd ../../
