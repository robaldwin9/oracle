#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd -- "$SCRIPT_DIR/.." && pwd)"

source "$SCRIPT_DIR/image-config.sh"

printf 'Building %s for linux/%s\n' "$IMAGE" "$ARCH"
printf 'Cross-architecture builds require QEMU/binfmt and may take a while.\n'

"$CONTAINER_ENGINE" build \
    --platform "linux/$ARCH" \
    --file "$PROJECT_DIR/Dockerfile" \
    --tag "$IMAGE" \
    "$PROJECT_DIR"

printf '\nBuilt %s\n' "$IMAGE"
printf 'Publish with the same environment: %q %q %q %q\n' \
    "$SCRIPT_DIR/publish.sh" "${1:-}" "$VERSION" "$ARCH"
