#!/bin/bash

set -e

echo "Debugging nix store..."
ls -al /nix-bin/store
find /nix-bin/store -type d -path '*/bin' | paste -sd: -
export PATH="/nix-bin/store:$(find /nix-bin/store -type d -path '*/bin' | paste -sd: -):$PATH"
echo "PATH: $PATH"

# Start Podman machine if not running
if ! podman machine inspect podman-machine-default --format '{{.State}}' 2>/dev/null | grep -q 'running'; then
    echo "Starting podman machine..."
    podman machine start
fi

# Wait for podman socket
timeout=30
elapsed=0
while [ ! -S "$DOCKER_HOST" ]; do
    echo "Waiting for podman socket..."
    sleep 1
    elapsed=$((elapsed + 1))
    if [ "$elapsed" -ge "$timeout" ]; then
        echo "Timed out waiting for podman socket."
        exit 1
    fi
done

echo "Podman socket ready. Launching application..."
exec /app/movement "$@"
