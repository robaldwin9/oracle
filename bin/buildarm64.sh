#!/usr/bin/env bash
set -euo pipefail

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
    printf 'Usage: %s [USERNAME] [VERSION]\nBuild an ARM64 image. Defaults: robaldwin, 0.1.0\n' \
        "$(basename "$0")"
    exit 0
fi

if (( $# > 2 )); then
    printf 'Too many arguments. Use --help for usage.\n' >&2
    exit 1
fi

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
exec "$SCRIPT_DIR/build.sh" "${1:-}" "${2:-0.1.0}" arm64
