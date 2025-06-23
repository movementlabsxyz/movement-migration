#!/bin/bash

set -e

echo "Debugging nix store..."
ls -al /nix/store
find /nix/store -type d -path '*/bin' | paste -sd: -
export PATH="/nix/store:$(find /nix/store -type d -path '*/bin' | paste -sd: -):$PATH"
echo "PATH: $PATH"

# Check if podman machine exists and is running
if ! podman machine inspect podman-machine-default &>/dev/null; then
    echo "Initializing podman machine..."
    podman machine init
    podman machine start
elif ! podman machine inspect podman-machine-default --format '{{.State}}' | grep -q 'running'; then
    echo "Starting podman machine..."
    podman machine start
fi

# Find the actual podman socket location
PODMAN_SOCKET=$(find /tmp/nix-shell.*/podman -name "podman-machine-default-api.sock" -type s 2>/dev/null | head -n 1)
if [ -n "$PODMAN_SOCKET" ]; then
    export DOCKER_HOST="unix://$PODMAN_SOCKET"
    echo "Set DOCKER_HOST to Podman socket: $DOCKER_HOST"
else
    echo "Warning: Could not find Podman socket"
fi

echo "Podman socket ready. Launching application..."
exec /app/mtma "$@"
