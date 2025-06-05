FROM nixos/nix:2.29.0 AS builder

RUN nix-env -iA nixpkgs.rsync nixpkgs.glibc nixpkgs.gawk

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

FROM nixos/nix:2.29.0

# Create a non-root user
RUN adduser -D movement

# Copy the build artifact and runtime dependencies
COPY --from=builder /tmp/build/target/release/mtma /app/mtma
COPY --from=builder /tmp/runtime/nix/store /nix/store

# Set up podman environment
ENV PATH="/nix/var/nix/profiles/default/bin:$PATH"
ENV XDG_RUNTIME_DIR="/run/user/1000"
ENV TMPDIR="/tmp"

# Create necessary directories with proper permissions
RUN mkdir -p /run/user/1000/podman && \
    chown -R movement:movement /run/user/1000 && \
    chown -R movement:movement /app

# Initialize podman machine during build
RUN if ! podman machine inspect podman-machine-default &>/dev/null; then \
        echo "Initializing podman machine..." && \
        podman machine init; \
    fi

# Switch to non-root user
USER movement

# Start podman machine and run the application
# You can't just RUN podman machine start because the socket will disappear once references are dropped, i.e., after the build completes.
ENTRYPOINT ["/bin/sh", "-c", "\
    if ! podman machine inspect podman-machine-default --format '{{.State}}' | grep -q 'running'; then \
        echo 'Starting podman machine...' && \
        podman machine start; \
    fi && \
    while [ ! -S /run/user/1000/podman/podman-machine-default-api.sock ]; do \
        echo 'Waiting for podman socket...' && \
        sleep 1; \
    done && \
    export DOCKER_HOST='unix:///run/user/1000/podman/podman-machine-default-api.sock' && \
    echo 'Set DOCKER_HOST to Podman socket: $DOCKER_HOST' && \
    exec /app/mtma \"$@\" \
"]