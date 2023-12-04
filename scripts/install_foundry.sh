#!/bin/bash

set -eu

echo "Installing foundryup"
curl -L https://foundry.paradigm.xyz | bash || echo "As expected, received a non-zero exit code"
export PATH="$PATH:/home/fabijanc/.foundry/bin"

# TODO if necessary, add the above PATH modification to BASH_ENV on circleci

echo "Installing foundry"
foundryup
anvil --version
