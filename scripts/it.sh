#!/bin/bash

assert_eq () {
  if [[ $1 == $2 ]]; then
    echo "Assertion succeeded: $1 == $2"
    return;
  fi
  echo "Assertion failed with: $1 == $2 "
  exit 1
}

# Build a release version of rhc
cargo build --release

# Run wordlist tests

target="zikazsanka666"

result=$(./target/release/rhc -p 5d6fb74aa3977bb2aaf565bd1b03d3ab3d654ee6bdbf88802e0cc902 --algorithm sha2_224 --wordlist resources/common.txt)
match=$(echo "$result" | sed -n 's/^Match: //p')
assert_eq $match $target

result=$(./target/release/rhc -p c8c860158e17d88b1e0d471f30f8f243bdaa8668b2bbc0e9b28ea84a4b4362e1 --algorithm sha2_256 --wordlist resources/common.txt)
match=$(echo "$result" | sed -n 's/^Match: //p')
assert_eq $match $target

result=$(./target/release/rhc -p 6ae586bd504e876f3cd84ff74da5d9ed2488dadbebe7d9cc5a7787ae750f1e2f3e89883d577d28b625b25542825ce194 --algorithm sha2_384 --wordlist resources/common.txt)
match=$(echo "$result" | sed -n 's/^Match: //p')
assert_eq $match $target

result=$(./target/release/rhc -p 712ae32218a7c68e0a35cb3b8ad5091b56d98b32e360da0ba11fa3f165874aa2cb1582679c1682085a5b7551942def6d5a076726257e5fddde9d9430eb3c5d5b --algorithm sha2_512 --wordlist resources/common.txt)
match=$(echo "$result" | sed -n 's/^Match: //p')
assert_eq $match $target

# Run brute force tests

target="abc"

result=$(./target/release/rhc -p 23097d223405d8228642a477bda255b32aadbce4bda0b3f7e36c9da7 --algorithm sha2_224)
match=$(echo "$result" | sed -n 's/^Match: //p')
assert_eq $match $target