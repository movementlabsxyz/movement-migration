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

# Extract socket path from DOCKER_HOST
if [[ "$DOCKER_HOST" == unix://* ]]; then
    SOCKET_PATH="${DOCKER_HOST#unix://}"
    echo "Socket path: $SOCKET_PATH"
    
    # Check if socket file exists and is accessible
    if [ ! -S "$SOCKET_PATH" ]; then
        echo "ERROR: Socket file does not exist or is not accessible: $SOCKET_PATH"
        echo "Please ensure the container runtime socket is available and mounted"
        exit 1
    fi
    
    # Test socket connectivity by trying to list containers
    echo "Testing socket connectivity..."
    if command -v docker &> /dev/null; then
        if docker ps &> /dev/null; then
            echo "✓ Docker socket is working - can list containers"
        else
            echo "✗ Docker socket test failed"
            exit 1
        fi
    elif command -v podman &> /dev/null; then
        if podman ps &> /dev/null; then
            echo "✓ Podman socket is working - can list containers"
        else
            echo "✗ Podman socket test failed"
            exit 1
        fi
    else
        echo "WARNING: Neither docker nor podman command found, but socket exists"
        echo "Socket validation passed, but container management may not work"
    fi
else
    echo "WARNING: DOCKER_HOST is not a unix socket, container management may not work"
    echo "Current DOCKER_HOST: $DOCKER_HOST"
fi

echo "Container management setup validated successfully!"
echo "Launching application..."
exec /app/movement "$@"
