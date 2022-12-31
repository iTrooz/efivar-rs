#!/bin/bash

set -xeuo pipefail

sed -i '/efiboot\|efivar/,/^version =/{s/version = ".*"/version = "'"$1"'"/}' efiboot/Cargo.toml efivar/Cargo.toml Cargo.lock

sed -i 's#efivar\/[0-9]*.[0-9]*.[0-9]*#efivar/'"$1"'#' efivar/src/lib.rs
