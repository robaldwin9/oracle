#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"

source "$SCRIPT_DIR/image-config.sh"

if ! ACTUAL_ARCH="$("$CONTAINER_ENGINE" image inspect \
    --format '{{.Architecture}}' "$IMAGE")"; then
    printf '\nImage not available locally: %s\n' "$IMAGE" >&2
    printf 'Build it first with the same environment: %q %q %q %q\n' \
        "$SCRIPT_DIR/build.sh" "${1:-}" "$VERSION" "$ARCH" >&2
    exit 1
fi

if [[ "$ACTUAL_ARCH" != "$ARCH" ]]; then
    printf 'Refusing to publish: expected %s, but image is %s.\n' \
        "$ARCH" "$ACTUAL_ARCH" >&2
    exit 1
fi

printf 'Publishing %s\n' "$IMAGE"
"$CONTAINER_ENGINE" push "$IMAGE"
printf '\nPublished %s\n' "$IMAGE"
