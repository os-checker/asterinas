#!/bin/bash

# SPDX-License-Identifier: MPL-2.0

SCRIPT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && cd ../.. && pwd)
VERSION_FILE="$SCRIPT_DIR/VERSION"
BOOK_SRC_DIR="$SCRIPT_DIR/book/src"

TARGET_VERSION=$(cat "$VERSION_FILE" | tr -d '[:space:]')
echo "Target version for validation: $TARGET_VERSION"

# Define the search pattern.
# This regex looks for 'asterinas.github.io/api-docs/'
# followed by any version number that IS NOT '${TARGET_VERSION}/'.
# (?!${TARGET_VERSION}/) is a Negative Lookahead.
PATTERN="asterinas\.github\.io/api-docs/(?!${TARGET_VERSION}/)[0-9]+\.[0-9]+\.[0-9]+"

echo "Scanning directory: ${BOOK_SRC_DIR}"

# Execute search:
# -r: recursive search
# -P: use Perl-Compatible Regular Expressions (required for negative lookahead)
# -n: show line numbers
MISMATCHES=$(grep -rPn "$PATTERN" "$BOOK_SRC_DIR")

# 7. Final Check
if [ -n "$MISMATCHES" ]; then
  echo "----------------------------------------------------------------"
  echo "❌ ERROR: Found links with outdated or incorrect versions:"
  echo "$MISMATCHES"
  echo "----------------------------------------------------------------"
  echo "Please update the links above to match version $TARGET_VERSION."
  exit 1
else
  echo "✅ SUCCESS: All found links match version $TARGET_VERSION."
  exit 0
fi
