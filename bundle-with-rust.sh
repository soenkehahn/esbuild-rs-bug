#!/usr/bin/env bash

set -eux

cd js-project
yarn install
cargo run
node bundled.js
