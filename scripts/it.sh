#!/bin/bash

BUILD_ENABLED=false
FAIL_EARLY=false

# Define ANSI escape codes for formatting output
bold=$(tput bold)
underline=$(tput smul)
red=$(tput setaf 1)
green=$(tput setaf 2)
reset=$(tput sgr0)

# Test counter
number=0;
# SuccessCounter
success=0;
# FailureCounter
failure=0;

# Set the tab length
tabs 4 

print_result() {
  if (( $success > 0 )); then
    echo "${green}${bold}Passed $success/$number tests${reset}"
  fi
  if (( $failure > 0 )); then
    echo "${red}${bold}Failed $failure/$number test${reset}"
    exit 1;
  fi
}

print_success() {
  if [[ $1 == *$'\n'* ]] || (( ${#1} > 20 )); then
    echo -e "\t${green}${bold}Assertion ($number) succeeded:${reset}"
    echo -e "\t${green}Got expected output:${reset}"
    echo -e "$(echo "$1"| sed -e 's/^/\t\t/')"
  else 
    echo -e "\t${green}${bold}Assertion ($number) succeeded:${reset} $1 == $1"
  fi
}

print_failure() {
  if [[ $1 == *$'\n'* ]] || (( ${#1} > 20 )); then
    echo -e "\t${red}${bold}Assertion ($number) failed with (arg1) == (arg2):${reset} "
    # TODO: This might be confusing if the issue is a newline character
    echo -e "$(echo "\t\t${red}(1)${reset}$1" | sed ':a;N;$!ba;s/\n/ /g')"
    echo -e "$(echo "\t\t${red}(2)${reset}$2" | sed ':a;N;$!ba;s/\n/ /g')"
  else 
    echo -e "\t${red}${bold}Assertion ($number) failed:${reset} $1 == $2"
  fi
}

assert_eq () {
  ((number++))
 if [[ $1 == $2 ]]; then
    ((success++))
    print_success "$1"
  else
    ((failure++))
    print_failure "$1" "$2"
    if $FAIL_EARLY; then 
      exit 1
    fi
  fi
}

# Loop over command-line arguments
for arg in "$@"
do
    case $arg in
        build)
            BUILD_ENABLED=true
            ;;
        fail-early)
            FAIL_EARLY=true
            ;;
        *)
            echo "Unknown argument: $arg"
            exit 1
            ;;
    esac
done

if [[ BUILD_ENABLED ]]; then
  # Build a release version of rhc
  cargo build --release
fi

if [[ ! -e "./target/release/rhc" ]]; then
  echo "Unable to find release build."
  cargo build --release
fi

# Run Dictionary tests
echo "${underline}${bold}Running Dictionary tests${reset}"
target="zikazsanka666"

result=$(./target/release/rhc -t 5d6fb74aa3977bb2aaf565bd1b03d3ab3d654ee6bdbf88802e0cc902 --algorithm sha2_224 --wordlist resources/common.txt)
match=$(echo "$result" | sed -n 's/^Decrypted: //p')
assert_eq $match $target

result=$(./target/release/rhc -t c8c860158e17d88b1e0d471f30f8f243bdaa8668b2bbc0e9b28ea84a4b4362e1 --algorithm sha2_256 --wordlist resources/common.txt)
match=$(echo "$result" | sed -n 's/^Decrypted: //p')
assert_eq $match $target

result=$(./target/release/rhc -t 6ae586bd504e876f3cd84ff74da5d9ed2488dadbebe7d9cc5a7787ae750f1e2f3e89883d577d28b625b25542825ce194 --algorithm sha2_384 --wordlist resources/common.txt)
match=$(echo "$result" | sed -n 's/^Decrypted: //p')
assert_eq $match $target

result=$(./target/release/rhc -t 712ae32218a7c68e0a35cb3b8ad5091b56d98b32e360da0ba11fa3f165874aa2cb1582679c1682085a5b7551942def6d5a076726257e5fddde9d9430eb3c5d5b --algorithm sha2_512 --wordlist resources/common.txt)
match=$(echo "$result" | sed -n 's/^Decrypted: //p')
assert_eq $match $target

# Run Incremental tests
echo "${underline}${bold}Running Incremental tests${reset}"
target="abc"

result=$(./target/release/rhc -t 23097d223405d8228642a477bda255b32aadbce4bda0b3f7e36c9da7 --algorithm sha2_224)
match=$(echo "$result" | sed -n 's/^Decrypted: //p')
assert_eq $match $target

result=$(./target/release/rhc -t ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad --algorithm sha2_256)
match=$(echo "$result" | sed -n 's/^Decrypted: //p')
assert_eq $match $target

result=$(./target/release/rhc -t cb00753f45a35e8bb5a03d699ac65007272c32ab0eded1631a8b605a43ff5bed8086072ba1e7cc2358baeca134c825a7 --algorithm sha2_384)
match=$(echo "$result" | sed -n 's/^Decrypted: //p')
assert_eq $match $target

result=$(./target/release/rhc -t ddaf35a193617abacc417349ae20413112e6fa4e89a97ea20a9eeee64b55d39a2192992a274fc1a836ba3c23a3feebbd454d4423643ce80e2a9ac94fa54ca49f --algorithm sha2_512)
match=$(echo "$result" | sed -n 's/^Decrypted: //p')
assert_eq $match $target

result=$(./target/release/rhc -t 4634270f707b6a54daae7530460842e20e37ed265ceee9a43e8924aa --algorithm sha2_512_224)
match=$(echo "$result" | sed -n 's/^Decrypted: //p')
assert_eq $match $target

result=$(./target/release/rhc -t 53048e2681941ef99b2e29b76b4c7dabe4c2d0c634fc6d46e0e2f13107e7af23 --algorithm sha2_512_256)
match=$(echo "$result" | sed -n 's/^Decrypted: //p')
assert_eq $match $target

# Run tests with specific flags
echo "${underline}${bold}Running specific flag tests${reset}"
target="abcd"

result=$(./target/release/rhc --target a76654d8e3550e9a2d67a0eeb6c67b220e5885eddd3fde135806e601 --min-length 4)
match=$(echo "$result" | sed -n 's/^Decrypted: //p')
assert_eq $match $target

result=$(./target/release/rhc --target a76654d8e3550e9a2d67a0eeb6c67b220e5885eddd3fde135806e601 --min-length 4 --max-length 5)
match=$(echo "$result" | sed -n 's/^Decrypted: //p')
assert_eq $match $target

result=$(./target/release/rhc --target a76654d8e3550e9a2d67a0eeb6c67b220e5885eddd3fde135806e601 --min-length 4 --max-length 5 --quiet)
match=$(echo "$result" | sed -n 's/^Decrypted: //p')
assert_eq $match $target

# Run tests with invalid flags / arguments
echo "${underline}${bold}Running invalid flags/arguments tests${reset}"

result=$(./target/release/rhc)
target=$'No argument specified.\nUse -h, --help for available options.'
assert_eq "$result" "$target"

result=$(./target/release/rhc --invalid-flag)
target=$'No such argument: --invalid-flag.\nUse -h, --help for available options.'
assert_eq "$result" "$target"

result=$(./target/release/rhc --min-length 4 --max-length 2)
target=$'No target supplied, unable to run.\nsee --target --help for information.'
assert_eq "$result" "$target"

result=$(./target/release/rhc -t bdd03d560993e675516ba5a50638b6531ac2ac3d5847c61916cfced6 --algorithm sha2_224 --min-length 4 --max-length 2)
target=$'Minimum length: "4" exceeds Maximum length: "2".'
assert_eq "$result" "$target"

result=$(./target/release/rhc --target a76654d8e3550e9a2d67a0eeb6c67b220e5885eddd3fde135806e601 --algorithm sha1)
target=$'"sha1" is not a suppported algorithm.\nUse --algorithm --help for available algorithms.'
assert_eq "$result" "$target"

result=$(./target/release/rhc --target malformed)
target=$'The input hash is malformed, unable to continue. Validate the hash and try again.'
assert_eq "$result" "$target"

result=$(./target/release/rhc --target bdd03d560993e675516ba5a50638b6531ac2ac3d5847c61916cfced6 --algorithm sha2_224 --thread-count 100)
target=$'Invalid thread count, not enough threads available.\nsee --threadcount --help for information.'
assert_eq "$result" "$target"

print_result