#! /bin/bash

# set -x
set -euo pipefail

if [[ $# -ne 1 ]]; then
    echo "$0: expect one argument: version" >&2
    exit 1
fi

VERSION="version = \"$1\""

SCRIPTS="$(dirname "$(realpath "$0")")"
WORKSPACE="$(realpath "$SCRIPTS"/..)"

cd "$WORKSPACE"

if ! scripts/check_changelog.sh refs/tags/v"$1"; then
    echo "$0: Please update CHANGELOG.md." >&2
    exit 1
fi

find . -name Cargo.toml -exec sed -i "{
s/^version = \"[^\"]*\"$/$VERSION/
}" {} \;

REQ="${VERSION/\"/\"=}"

find . -name Cargo.toml -exec sed -i "/^cast_checks/{
s/^\(.*\)\<version = \"[^\"]*\"\(.*\)$/\1$REQ\2/
}" {} \;

scripts/update_lockfiles.sh
