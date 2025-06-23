FROM nixos/nix:2.29.0 AS builder

RUN nix-env -iA nixpkgs.rsync nixpkgs.glibc nixpkgs.gawk

# Copy the source code into the container
COPY . /tmp/build
WORKDIR /tmp/build

# Set build to docker to skip the podman initilization while opening the flake
ENV BUILD=docker

RUN nix --extra-experimental-features "nix-command flakes" \
        develop --command bash -c "mkdir -p  /tmp/executables"

RUN nix --extra-experimental-features "nix-command flakes" \
        develop --command bash -c "cp \$(which podman) /tmp/executables/podman"

RUN nix --extra-experimental-features "nix-command flakes" \
        develop --command bash -c "cp \$(which docker) /tmp/executables/docker"

# Build the Rust application
RUN nix --extra-experimental-features "nix-command flakes" \
        develop .#docker-build --command bash -c "cargo build --release --bin movement"


RUN rust_binary="./target/release/movement"; dest_dir="/tmp/runtime"; \
    mkdir -p "$dest_dir"; ldd "$rust_binary" | awk '{print $3}' | \
    grep '^/' | xargs -I {} dirname {} | sort | uniq | xargs -I {} \
    bash -c 'mkdir -p "$0/$1" && rsync -a --copy-links "$1/" "$0/$1/"' "$dest_dir" {}

RUN rust_binary="/tmp/executables/podman"; dest_dir="/tmp/runtime"; \
    mkdir -p "$dest_dir"; ldd "$rust_binary" | awk '{print $3}' | \
    grep '^/' | xargs -I {} dirname {} | sort | uniq | xargs -I {} \
    bash -c 'mkdir -p "$0/$1" && rsync -a --copy-links "$1/" "$0/$1/"' "$dest_dir" {}

FROM alpine:3.22.0

RUN apk add --no-cache bash

# Create non-root user
RUN adduser -u 1000 -D -s /bin/bash movement

# Copy binary and runtime deps
COPY --from=builder /tmp/build/target/release/movement /app/movement
COPY --from=builder /nix/store /nix/store

# copy executables
COPY --from=builder /tmp/executables/podman /usr/local/bin/podman
COPY --from=builder /tmp/executables/docker /usr/local/bin/docker

# Environment setup
ENV PATH="/nix/var/nix/profiles/default/bin:$PATH"
ENV XDG_RUNTIME_DIR="/run/user/1000"
ENV TMPDIR="/tmp"

# Create required runtime dirs with proper ownership
RUN mkdir -p /run/user/1000 && \
    chown -R movement:movement /run/user/1000 /app /nix

# Copy runtime bootstrap script
COPY docker/build/movement/entry.sh /app/entry.sh
RUN chmod +x /app/entry.sh && chown movement:movement /app/entry.sh

# Switch to non-root user
USER movement

# Entrypoint to bootstrap podman and launch movement
ENTRYPOINT ["/app/entry.sh"]