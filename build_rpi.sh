#!/usr/bin/env bash
cross build --bin lebron --target aarch64-unknown-linux-gnu --release
scp target/aarch64-unknown-linux-gnu/release/lebron simon@steward.local:
