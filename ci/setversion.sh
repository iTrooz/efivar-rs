#!/bin/bash

set -xeuo pipefail

# Version specifications in Cargo.toml/lock
sed -i '/efiboot\|efivar/{n;s/^version = "[^"]*"/version = "'"$1"'"/}' efiboot/Cargo.toml efivar/Cargo.toml Cargo.lock

# Dependency to efivar
sed -i '/efivar\s*=/{s/version = "[^"]*"/version = "'"$1"'"/}' efiboot/Cargo.toml Cargo.lock

# doc html_root_url attribute
sed -i 's#efivar\/[^"]*#efivar/'"$1"'#' efivar/src/lib.rs
