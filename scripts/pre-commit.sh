#!/usr/bin/env bash
if [[ $(dirname "${BASH_SOURCE[0]}") != *".git/hooks" ]]; then
  return
fi

if command -v cargo &>/dev/null; then
  cargo fmt
  git add -A
  echo "cargo fmt finished"
fi
