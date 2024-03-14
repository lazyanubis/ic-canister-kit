#!/bin/bash

cargo clippy

cargo audit --quiet

cargo build
