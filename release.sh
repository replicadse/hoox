#!/bin/bash

VERSION=$1

if [ -z "$VERSION" ]; then
  echo "Usage: release.sh <version>"
  exit 1
fi

cp Cargo.toml Cargo.toml.bak
sed 's/version = "0.0.0"/version = "'$VERSION'"/g' Cargo.toml > Cargo.toml.tmp
mv Cargo.toml.tmp Cargo.toml

if read -t 10 -p "Do you want to continue releasing version $VERSION? (Y/n)? " response; then
  if [ "$response" == "Y" ]; then
    echo "Releasing version $VERSION"
    cargo publish --allow-dirty
  else
    echo "Aborted"
  fi
else
  echo ''
  echo "Aborted"
fi

mv Cargo.toml.bak Cargo.toml
