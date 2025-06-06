FROM debian:bullseye-slim AS builder

# System setup
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y \
    curl \
    sudo \
    git \
    bash \
    ca-certificates \
    build-essential \
    libssl-dev \
    lsb-release \
    gnupg \
    && rm -rf /var/lib/apt/lists/*

# Add builder user
RUN useradd -ms /bin/bash builder && echo "builder ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

# Install Nix (multi-user, Determinate installer)
RUN curl --proto '=https' --tlsv1.2 -sSf https://install.determinate.systems/nix | bash -s -- install --no-confirm

# Make nix available in non-login shell
ENV PATH="/nix/var/nix/profiles/default/bin:$PATH"
ENV NIX_PATH=/nix/var/nix/profiles/per-user/root/channels
ENV USER=builder

# Set up nix for non-root user
USER builder
WORKDIR /home/builder
ENV PATH="/home/builder/.nix-profile/bin:/nix/var/nix/profiles/default/bin:$PATH"
SHELL ["/bin/bash", "-c"]

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