#!/bin/bash

# Read the current version
version=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml)

# Increment the version
IFS='.' read -r -a array <<< "$version"
if [[ "$1" == "patch" ]]; then
  array[2]=$((array[2]+1))
elif [[ "$1" == "minor" ]]; then
  array[1]=$((array[1]+1))
elif [[ "$1" == "major" ]]; then
  array[0]=$((array[0]+1))
else 
  echo "requires one of minor, patch, major"
  exit 1
fi

new_version="${array[0]}.${array[1]}.${array[2]}"

# Update the version in Cargo.toml
sed -i "s/^version = \"$version\"/version = \"$new_version\"/" Cargo.toml
# Update the version in Cargo.lock
sed -i "/^\[\[package\]\]$/,/^$/{/^name = \"rhc\"$/{n;s/^version = \".*\"/version = \"$new_version\"/;}}" Cargo.lock

