FROM nixos/nix:2.29.0 AS builder

RUN nix-env -iA nixpkgs.rsync nixpkgs.glibc nixpkgs.gawk

# Copy the source code into the container
COPY . /tmp/build
WORKDIR /tmp/build

# Set build to docker to skip the podman initilization while opening the flake
ENV BUILD=docker

# Build the Rust application
RUN nix --extra-experimental-features "nix-command flakes" \
        develop .#docker-build --command bash -c "cargo build --release -p movement"

RUN rust_binary="./target/release/movement"; dest_dir="/tmp/runtime"; \
    mkdir -p "$dest_dir"; ldd "$rust_binary" | awk '{print $3}' | \
    grep '^/' | xargs -I {} dirname {} | sort | uniq | xargs -I {} \
    bash -c 'mkdir -p "$0/$1" && rsync -a --copy-links "$1/" "$0/$1/"' "$dest_dir" {}


FROM alpine:3.22.0

# Create non-root user
RUN adduser -u 1000 -D -s /bin/bash movement

# Copy binary and runtime deps
COPY --from=builder /tmp/build/target/release/movement /app/movement
COPY --from=builder /tmp/runtime/nix/store /nix/store

# Environment setup
ENV PATH="/nix/var/nix/profiles/default/bin:$PATH"
ENV XDG_RUNTIME_DIR="/run/user/1000"
ENV TMPDIR="/tmp"
ENV DOCKER_HOST="unix:///run/user/1000/podman/podman-machine-default-api.sock"

# Create required runtime dirs with proper ownership
RUN mkdir -p /run/user/1000/podman && \
    chown -R movement:movement /run/user/1000 /app /nix

# Copy runtime bootstrap script
COPY docker/build/movement/entry.sh /app/entry.sh
RUN chmod +x /app/entry.sh

# Switch to non-root user
USER movement

# Manually pull the needed images
# TODO: this should be refactored s.t. we can get the list from the movement command. But, probably not in this PR.
#   "movement-full-node"
RUN podman pull ghcr.io/movementlabsxyz/movement-full-node:c2372ff
# "movement-celestia-da-light-node"
RUN podman pull ghcr.io/movementlabsxyz/movement-celestia-da-light-node:c2372ff
# "movement-full-node-setup"
RUN podman pull ghcr.io/movementlabsxyz/movement-full-node-setup:c2372ff
# "movement-faucet-service"
RUN podman pull ghcr.io/movementlabsxyz/movement-faucet-service:c2372ff
# "movement-celestia-bridge"
RUN podman pull ghcr.io/movementlabsxyz/movement-celestia-bridge:c2372ff
# "movement-celestia-appd"
RUN podman pull ghcr.io/movementlabsxyz/movement-celestia-appd:c2372ff
# "wait-for-celestia-light-node"
RUN podman pull ghcr.io/movementlabsxyz/wait-for-celestia-light-node:c2372ff

# Entrypoint to bootstrap podman and launch movement
ENTRYPOINT ["/app/entry.sh"]