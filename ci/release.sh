#!/bin/bash

set -euo pipefail

set_version () {
    # Version specifications in Cargo.toml/lock
    sed -i '/efiboot\|efivar/{n;s/^version = "[^"]*"/version = "'"$1"'"/}' efiboot/Cargo.toml efivar/Cargo.toml Cargo.lock

    # Dependency to efivar
    sed -i '/efivar\s*=/{s/version = "[^"]*"/version = "'"$1"'"/}' efiboot/Cargo.toml Cargo.lock

    # doc html_root_url attribute
    sed -i 's#efivar\/[^"]*#efivar/'"$1"'#' efivar/src/lib.rs
}

publish_crate () {
    local crate_name="$1"
    local max_tries=6

    for i in $(seq 0 $max_tries); do
        echo "Try $i out of $max_tries"

        if cargo publish -p "$crate_name"; then
            echo "Succeeded, moving on."
            break
        else
            echo "Failed, waiting 10s."
            sleep 10
        fi
    done
}

publish () {
    publish_crate efivar
    publish_crate efiboot
}

action="$1"
shift

case "$action" in
    set-version)
        set_version "$@"
        ;;
    publish)
        publish
        ;;
    *)
        echo "Usage: ./ci/release.sh set-version|publish"
        ;;
esac
