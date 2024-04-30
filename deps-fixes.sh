#!/usr/bin/env bash
# These will need to be periodically updated or removed once upstream dependencies update their versions

cargo update -p openssl --precise 0.10.64
cargo update -p ahash --precise 0.8.11
cargo update -p rustix --precise 0.36.16
