#!/bin/bash

set -e

cd rust

echo "Running benchmark"

cargo bench

set +e

cd ../../
