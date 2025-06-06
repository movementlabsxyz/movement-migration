FROM ubuntu:24.10 AS builder

# Create non-root user early
RUN useradd -ms /bin/bash builder

# Install curl and other deps as root BEFORE switching users
RUN apt-get update && apt-get install -y curl systemd ca-certificates && rm -rf /var/lib/apt/lists/*

USER builder
ENV USER=builder
WORKDIR /home/builder

# Install Determinate Nix as builder (non-root setup)
RUN curl --proto '=https' --tlsv1.2 -sSf https://install.determinate.systems/nix | \
    bash -s -- install linux \
    --extra-conf "sandbox = false" \
    --no-start-daemon \
    --no-confirm

# Set up proper Nix PATH for non-root user
ENV PATH="/home/builder/.nix-profile/bin:/nix/var/nix/profiles/default/bin:$PATH"

# Prove it works
RUN nix run nixpkgs#hello

# Copy the source code into the container
COPY . /tmp/build
WORKDIR /tmp/build

# Build the Rust application
RUN nix --extra-experimental-features "nix-command flakes" \
        develop .#docker-build --command bash -c "cargo build --release -p mtma"

RUN rust_binary="./target/release/mtma"; dest_dir="/tmp/runtime"; \
    mkdir -p "$dest_dir"; ldd "$rust_binary" | awk '{print $3}' | \
    grep '^/' | xargs -I {} dirname {} | sort | uniq | xargs -I {} \
    bash -c 'mkdir -p "$0/$1" && rsync -a --copy-links "$1/" "$0/$1/"' "$dest_dir" {}

FROM alpine:3.22.0

# Create non-root user
RUN adduser -u 1000 -D -s /bin/bash mtma

# Copy binary and runtime deps
COPY --from=builder /tmp/build/target/release/mtma /app/mtma
COPY --from=builder /tmp/runtime/nix/store /nix/store

# Environment setup
ENV PATH="/nix/var/nix/profiles/default/bin:$PATH"
ENV XDG_RUNTIME_DIR="/run/user/1000"
ENV TMPDIR="/tmp"
ENV DOCKER_HOST="unix:///run/user/1000/podman/podman-machine-default-api.sock"

# Create required runtime dirs with proper ownership
RUN mkdir -p /run/user/1000/podman && \
    chown -R mtma:mtma /run/user/1000 /app /nix

# Copy runtime bootstrap script
COPY docker/build/mtma/entry.sh /app/entry.sh
RUN chmod +x /app/entry.sh

# Switch to non-root user
USER mtma

# Entrypoint to bootstrap podman and launch mtma
ENTRYPOINT ["/app/entry.sh"]