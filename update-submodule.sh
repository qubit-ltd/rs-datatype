#!/bin/bash
################################################################################
#
#    Copyright (c) 2025 - 2026 Haixing Hu.
#
#    SPDX-License-Identifier: Apache-2.0
#
#    Licensed under the Apache License, Version 2.0.
#
################################################################################
#
# Sync and update Git submodules from the repository root.
# Run from repo root: ./update-submodule.sh
# By default, updates submodules to the latest commit on their remote tracking
# branches.
#

set -euo pipefail

usage() {
    cat <<'EOF_USAGE'
Usage: ./update-submodule.sh [options]

Initialize `.rs-ci` when needed, then switch it to local `main` and update it
to the latest `origin/main` commit.

Options:
  --shallow     Shallow clone (passes --depth 1 to git submodule update)
  -h, --help    Show this help

Environment:
  GIT_SUBMODULE_DEPTH   If set to 1, same as --shallow
EOF_USAGE
}

require_command() {
    if ! command -v "$1" > /dev/null 2>&1; then
        echo "error: required command '$1' was not found" >&2
        exit 1
    fi
}

PROJECT_ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
cd "$PROJECT_ROOT"

shallow=0
while [ "$#" -gt 0 ]; do
    case "$1" in
        --shallow)
            shallow=1
            ;;
        -h | --help)
            usage
            exit 0
            ;;
        *)
            echo "error: unknown argument: $1" >&2
            usage >&2
            exit 1
            ;;
    esac
    shift
done

if [ "${GIT_SUBMODULE_DEPTH:-}" = "1" ]; then
    shallow=1
fi

require_command git

if [ ! -f .gitmodules ]; then
    echo "error: .gitmodules not found in the current directory; cannot update submodules" >&2
    exit 1
fi

echo "==> git submodule sync --recursive"
git submodule sync --recursive

SUBMODULE_PATH=".rs-ci"
SUBMODULE_GIT_DIR="$PROJECT_ROOT/$SUBMODULE_PATH"

if ! git -C "$SUBMODULE_GIT_DIR" rev-parse --is-inside-work-tree > /dev/null 2>&1; then
    update_args=(submodule update --init --recursive)
    if [ "$shallow" -eq 1 ]; then
        update_args+=(--depth 1)
    fi
    update_args+=("$SUBMODULE_PATH")

    echo "==> git ${update_args[*]}"
    git "${update_args[@]}"
else
    echo "==> submodule '$SUBMODULE_PATH' is already initialized"
fi

if ! git -C "$SUBMODULE_GIT_DIR" rev-parse --is-inside-work-tree > /dev/null 2>&1; then
    echo "error: submodule '$SUBMODULE_PATH' is not a Git working tree after initialization" >&2
    exit 1
fi

if [ -n "$(git -C "$SUBMODULE_GIT_DIR" status --porcelain --untracked-files=all)" ]; then
    echo "error: submodule '$SUBMODULE_PATH' has uncommitted changes; refusing to switch or update it" >&2
    exit 1
fi

echo "==> git -C $SUBMODULE_PATH fetch --prune origin main"
git -C "$SUBMODULE_GIT_DIR" fetch --prune origin \
    '+refs/heads/main:refs/remotes/origin/main'

if ! git -C "$SUBMODULE_GIT_DIR" show-ref --verify --quiet refs/remotes/origin/main; then
    echo "error: submodule '$SUBMODULE_PATH' remote 'origin' has no main branch" >&2
    exit 1
fi

remote_main=$(git -C "$SUBMODULE_GIT_DIR" rev-parse refs/remotes/origin/main)
if git -C "$SUBMODULE_GIT_DIR" show-ref --verify --quiet refs/heads/main; then
    local_main=$(git -C "$SUBMODULE_GIT_DIR" rev-parse refs/heads/main)
    if ! git -C "$SUBMODULE_GIT_DIR" merge-base --is-ancestor "$local_main" "$remote_main"; then
        if git -C "$SUBMODULE_GIT_DIR" merge-base --is-ancestor "$remote_main" "$local_main"; then
            echo "error: submodule '$SUBMODULE_PATH' local main is ahead of origin/main; refusing to discard local commits" >&2
        else
            echo "error: submodule '$SUBMODULE_PATH' local main has diverged from origin/main; resolve the history manually" >&2
        fi
        exit 1
    fi

    echo "==> git -C $SUBMODULE_PATH switch main"
    git -C "$SUBMODULE_GIT_DIR" switch main
    git -C "$SUBMODULE_GIT_DIR" branch --set-upstream-to=origin/main main
    if [ "$local_main" != "$remote_main" ]; then
        echo "==> git -C $SUBMODULE_PATH merge --ff-only origin/main"
        git -C "$SUBMODULE_GIT_DIR" merge --ff-only origin/main
    fi
else
    echo "==> git -C $SUBMODULE_PATH switch --create main --track origin/main"
    git -C "$SUBMODULE_GIT_DIR" switch --create main --track origin/main
fi

echo "==> git -C $SUBMODULE_PATH submodule update --init --recursive"
git -C "$SUBMODULE_GIT_DIR" submodule update --init --recursive

echo "Done."
