#!/bin/sh
# Use the hooks in .githooks for this clone: pre-commit runs rustfmt and
# clippy, pre-push runs the tests.
set -e

cd "$(git rev-parse --show-toplevel)"
git config core.hooksPath .githooks
chmod +x .githooks/pre-commit .githooks/pre-push
echo "git hooks installed (core.hooksPath = .githooks)"
