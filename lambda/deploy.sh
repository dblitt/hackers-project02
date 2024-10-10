#!/bin/bash

set -euo pipefail

BINARY_NAME="barbuddy-lambda"
LAMBDA_NAME="testbarbuddygettest"

# File containing `export AWS_ACCESS_KEY_ID="..."` and AWS_SECRET_ACCESS_KEY
. ./AWS_CREDS

cargo lambda build --release --arm64
cargo lambda deploy --binary-name "$BINARY_NAME" "$LAMBDA_NAME"
