#!/bin/bash
number=0;
tabs 4 
assert_eq () {
  ((number++))
  if [[ $1 == $2 ]]; then
    echo -e "\tAssertion ($number) succeeded: $1 == $2"
    return;
  fi
  echo "Assertion ($number) failed with: $1 == $2 "
  exit 1
}

# Build a release version of rhc
cargo build --release

# Run dictionary tests
echo "Running Dictionary tests"
target="zikazsanka666"

result=$(./target/release/rhc -t 5d6fb74aa3977bb2aaf565bd1b03d3ab3d654ee6bdbf88802e0cc902 --algorithm sha2_224 --wordlist resources/common.txt)
match=$(echo "$result" | sed -n 's/^Match: //p')
assert_eq $match $target

result=$(./target/release/rhc -t c8c860158e17d88b1e0d471f30f8f243bdaa8668b2bbc0e9b28ea84a4b4362e1 --algorithm sha2_256 --wordlist resources/common.txt)
match=$(echo "$result" | sed -n 's/^Match: //p')
assert_eq $match $target

result=$(./target/release/rhc -t 6ae586bd504e876f3cd84ff74da5d9ed2488dadbebe7d9cc5a7787ae750f1e2f3e89883d577d28b625b25542825ce194 --algorithm sha2_384 --wordlist resources/common.txt)
match=$(echo "$result" | sed -n 's/^Match: //p')
assert_eq $match $target

result=$(./target/release/rhc -t 712ae32218a7c68e0a35cb3b8ad5091b56d98b32e360da0ba11fa3f165874aa2cb1582679c1682085a5b7551942def6d5a076726257e5fddde9d9430eb3c5d5b --algorithm sha2_512 --wordlist resources/common.txt)
match=$(echo "$result" | sed -n 's/^Match: //p')
assert_eq $match $target

# Run incremental tests
echo "Running Incremental tests"
target="abcde"

result=$(./target/release/rhc -t bdd03d560993e675516ba5a50638b6531ac2ac3d5847c61916cfced6 --algorithm sha2_224)
match=$(echo "$result" | sed -n 's/^Match: //p')
assert_eq $match $target

result=$(./target/release/rhc -t 36bbe50ed96841d10443bcb670d6554f0a34b761be67ec9c4a8ad2c0c44ca42c --algorithm sha2_256)
match=$(echo "$result" | sed -n 's/^Match: //p')
assert_eq $match $target

result=$(./target/release/rhc -t 4c525cbeac729eaf4b4665815bc5db0c84fe6300068a727cf74e2813521565abc0ec57a37ee4d8be89d097c0d2ad52f0 --algorithm sha2_384)
match=$(echo "$result" | sed -n 's/^Match: //p')
assert_eq $match $target

result=$(./target/release/rhc -t 878ae65a92e86cac011a570d4c30a7eaec442b85ce8eca0c2952b5e3cc0628c2e79d889ad4d5c7c626986d452dd86374b6ffaa7cd8b67665bef2289a5c70b0a1 --algorithm sha2_512)
match=$(echo "$result" | sed -n 's/^Match: //p')
assert_eq $match $target