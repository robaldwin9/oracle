#!/usr/bin/env bash
# Shared configuration sourced by build.sh and publish.sh.

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
    printf 'Usage: %s [USERNAME] [VERSION] [ARCH]\n' "$(basename "$0")"
    printf '\nDefaults: USERNAME=robaldwin, VERSION=0.1.0, ARCH=local system architecture\n'
    printf 'Supported architectures: arm64, amd64\n'
    printf '\nEnvironment overrides:\n'
    printf '  IMAGE_REPOSITORY  Override repository when USERNAME is omitted or empty\n'
    printf '  CONTAINER_ENGINE  Default: podman (also supports docker)\n'
    exit 0
fi

if (( $# > 3 )); then
    printf 'Too many arguments. Use --help for usage.\n' >&2
    exit 1
fi

DOCKERHUB_USERNAME="${1:-robaldwin}"
VERSION="${2:-0.1.0}"
ARCH="${3:-}"
CONTAINER_ENGINE="${CONTAINER_ENGINE:-podman}"

if [[ ! "$DOCKERHUB_USERNAME" =~ ^[a-z0-9][a-z0-9_-]*$ ]]; then
    printf 'Invalid Docker Hub username: %s. Use a lowercase username, not an email.\n' \
        "$DOCKERHUB_USERNAME" >&2
    exit 1
fi

if [[ -n "${1:-}" ]]; then
    IMAGE_REPOSITORY="docker.io/${DOCKERHUB_USERNAME}/oracle"
else
    IMAGE_REPOSITORY="${IMAGE_REPOSITORY:-docker.io/${DOCKERHUB_USERNAME}/oracle}"
fi

if [[ -z "$ARCH" ]]; then
    SYSTEM_ARCH="$(uname -m)"
    case "$SYSTEM_ARCH" in
        x86_64|amd64) ARCH=amd64 ;;
        aarch64|arm64) ARCH=arm64 ;;
        *)
            printf 'Unsupported system architecture: %s. Specify arm64 or amd64.\n' \
                "$SYSTEM_ARCH" >&2
            exit 1
            ;;
    esac
fi

case "$ARCH" in
    arm64|amd64) ;;
    *)
        printf 'Unsupported architecture: %s. Use arm64 or amd64.\n' "$ARCH" >&2
        exit 1
        ;;
esac

if [[ ! "$VERSION" =~ ^[A-Za-z0-9_][A-Za-z0-9_.-]*$ ]] \
    || (( ${#VERSION} + ${#ARCH} + 1 > 128 )); then
    printf 'Invalid image version: %s\n' "$VERSION" >&2
    exit 1
fi

if ! command -v "$CONTAINER_ENGINE" >/dev/null 2>&1; then
    printf 'Container engine not found: %s\n' "$CONTAINER_ENGINE" >&2
    exit 1
fi

IMAGE="${IMAGE_REPOSITORY}:${VERSION}-${ARCH}"
