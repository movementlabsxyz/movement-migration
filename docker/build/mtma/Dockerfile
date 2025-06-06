FROM nixos/nix:2.29.0 AS builder

# Install necessary packages and set up system files
RUN nix-env -iA nixpkgs.rsync nixpkgs.glibc nixpkgs.gawk nixpkgs.bash nixpkgs.shadow
RUN mkdir -p /etc && touch /etc/passwd /etc/group /etc/shadow /etc/gshadow
RUN chmod 644 /etc/passwd /etc/group && chmod 600 /etc/shadow /etc/gshadow

# Find bash location and create user
RUN BASH_PATH=$(which bash) && \
    echo "Using bash at: $BASH_PATH" && \
    useradd -s "$BASH_PATH" builder

# Switch to non-root user
USER builder

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