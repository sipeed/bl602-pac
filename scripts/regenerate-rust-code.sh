#!/usr/bin/env bash

# This script generates the Rust library code from the bl602 SVD file.

fail() {
  echo "Error: $*"
  exit 1
}

if [ $# -lt 2 ]; then
  echo "Usage: $0 <path to svd> <path to src output dir>"
  exit 1
fi

command -v svd2rust > /dev/null || fail "Please install rust2vd (https://github.com/rust-embedded/svd2rust)"
command -v form > /dev/null || fail "Please install form (https://github.com/djmcgill/form)"
command -v rustfmt > /dev/null || fail "Please install rustfmt"

svd_path="${1}"
src_path="${2}"

if [ ! -d "${src_path}" ]; then
  fail "src output dir (\`${src_path}') is not a directory"
fi

if [ ! -f "${src_path}/lib.rs" ]; then
  fail "The src output dir (\`${src_path}') does not contain a lib.rs - exiting to avoid accidentally deleting files"
fi

set -x

# Remove the existing code in the src directory
rm -v -rf "${src_path}/"

# Generate the new code
mkdir -v "${src_path}/"

# Generate the Rust code from the SVD
svd2rust -i $svd_path --target riscv

# Split the single generated lib.rs file into Rust modules
form -i lib.rs -o "${src_path}/" && rm lib.rs

# Reformat the code with rustfmt
find "${src_path}/" -name \*.rs -exec rustfmt -v {} \;
