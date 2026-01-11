#!/usr/bin/env sh
set -e

echo "Pulling main repo..."
git pull

echo "Updating submodules..."
git submodule update --init --recursive --remote

echo "Done."
