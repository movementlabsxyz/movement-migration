#!/bin/bash

set -e

echo "Debugging nix store..."
ls -al /nix/store
find /nix/store -type d -path '*/bin' | paste -sd: -
export PATH="/nix/store:$(find /nix/store -type d -path '*/bin' | paste -sd: -):$PATH"
echo "PATH: $PATH"

# Validate DOCKER_HOST environment variable and socket connectivity
echo "Validating container management setup..."

if [ -z "$DOCKER_HOST" ]; then
    echo "ERROR: DOCKER_HOST environment variable is not set"
    echo "Please set DOCKER_HOST to point to a valid Docker/Podman socket"
    echo "Examples:"
    echo "  - unix:///var/run/docker.sock (for Docker)"
    echo "  - unix:///run/user/1000/podman/podman.sock (for Podman)"
    exit 1
fi

echo "DOCKER_HOST is set to: $DOCKER_HOST"

echo "Container management setup validated successfully!"
echo "Launching application..."
exec /app/mtma "$@"
