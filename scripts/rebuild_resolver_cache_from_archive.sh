#!/usr/bin/env bash
set -euo pipefail

archive_dir="${RAW_ARCHIVE_DIR:-.raw-archive}"
from="${BACKFILL_FROM:-}"
to="${BACKFILL_TO:-}"

args=(archive-resolvers --archive-dir "$archive_dir")
if [[ -n "$from" ]]; then
  args+=(--from "$from")
fi
if [[ -n "$to" ]]; then
  args+=(--to "$to")
fi

cargo run -p cli -- "${args[@]}"
