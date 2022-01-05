#!/usr/bin/env bash

set -eux

cd js-project
yarn install
yarn esbuild -- --platform=node main.js --bundle --outfile=bundled.js
node bundled.js
