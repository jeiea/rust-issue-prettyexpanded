#!/bin/bash

cargo rustc --bin original -- -Z unstable-options --pretty=expanded

cargo build --bin expanded