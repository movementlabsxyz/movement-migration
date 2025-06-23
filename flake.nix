{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/a7abebc31a8f60011277437e000eebcc01702b9f";
    rust-overlay.url = "github:oxalica/rust-overlay/47beae969336c05e892e1e4a9dbaac9593de34ab";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    movement.url = "github:movementlabsxyz/movement/278863e49276f280fb7c719c5fddeb3826fe85ee";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, crane, movement, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        toolchain = p: (p.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
          extensions = [ "rustfmt" "clippy" ];
        };
        craneLib = (crane.mkLib pkgs).overrideToolchain(toolchain);

        frameworks = pkgs.darwin.apple_sdk.frameworks;

        # Create a proper Nix derivation for the movement repository
        movementRepo = pkgs.stdenv.mkDerivation {
          name = "movement-repo";
          src = movement;
          dontBuild = true;
          installPhase = ''
            mkdir -p $out
            cp -r . $out/
          '';
        };

        # An LLVM build environment
        buildDependencies = with pkgs; [
          perl
          llvmPackages.bintools
          openssl
          openssl.dev
          libiconv 
          pkg-config
          libclang.lib
          libz
          clang
          pkg-config
          protobuf
          rustPlatform.bindgenHook
          lld
          coreutils
          gcc
          rust
          zlib
          pandoc
          postgresql
          qemu_kvm
          qemu-utils
          libvirt
          cargo-machete
        ] ++ lib.optionals stdenv.isDarwin [
          fixDarwinDylibNames
        ] ++ lib.optionals stdenv.isLinux [
          virtiofsd
        ];
        
        sysDependencies = with pkgs; [] 
        ++ lib.optionals stdenv.isDarwin [
          frameworks.Security
          frameworks.CoreServices
          frameworks.SystemConfiguration
          frameworks.AppKit
          libelf
        ] ++ lib.optionals stdenv.isLinux [
          udev
          systemd
          bzip2
          elfutils
          jemalloc
        ];

        testDependencies = with pkgs; [
          python311
          just
          process-compose
          jq
          docker
          podman
          solc
          grpcurl
          grpcui
        ];

        # Specific version of toolchain
        rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rust;
          rustc = rust;
        };
    
      in {
        devShells = rec {
          default = docker-build;
          docker-build = pkgs.mkShell {
            ROCKSDB = pkgs.rocksdb;
            OPENSSL_DEV = pkgs.openssl.dev;

            hardeningDisable = ["fortify"];

            buildInputs = with pkgs; [
              # rust toolchain
              (toolchain pkgs)
            ] ++ sysDependencies ++ buildDependencies ++ testDependencies;

            LD_LIBRARY_PATH = "${pkgs.stdenv.cc.cc.lib}/lib/";

            shellHook = ''
              #!/usr/bin/env ${pkgs.bash}

              set -e

              # Export linker flags if on Darwin (macOS)
              if [[ "${pkgs.stdenv.hostPlatform.system}" =~ "darwin" ]]; then
                export MACOSX_DEPLOYMENT_TARGET=$(sw_vers -productVersion)
                export LDFLAGS="-L/opt/homebrew/opt/zlib/lib"
                export CPPFLAGS="-I/opt/homebrew/opt/zlib/include"
              fi

              # Always force recreation of the podman machine
              if podman machine inspect podman-machine-default &>/dev/null; then
                echo "Destroying existing podman machine..."
                if ! podman machine stop podman-machine-default 2>podman_stop_err.log; then
                  echo "podman machine stop failed. Error output:" >&2
                  cat podman_stop_err.log >&2
                fi
                if ! podman machine rm podman-machine-default -f 2>podman_rm_err.log; then
                  echo "podman machine rm failed. Error output:" >&2
                  cat podman_rm_err.log >&2
                  exit 1
                fi
              fi

              echo "Initializing podman machine..."
              if ! podman machine init 2>podman_init_err.log; then
                echo "podman machine init failed. Error output:" >&2
                cat podman_init_err.log >&2
                exit 1
              fi
              if ! podman machine start 2>podman_start_err.log; then
                echo "podman machine start failed. Error output:" >&2
                cat podman_start_err.log >&2
                exit 1
              fi

              echo "Podman machine is running"

              # Find the actual podman socket location
              PODMAN_SOCKET=$(find /tmp/nix-shell.*/podman -name "podman-machine-default-api.sock" -type s 2>/dev/null | head -n 1)
              if [ -n "$PODMAN_SOCKET" ]; then
                export DOCKER_HOST="unix://$PODMAN_SOCKET"
                echo "Set DOCKER_HOST to Podman socket: $DOCKER_HOST"
              else
                echo "Warning: Could not find Podman socket"
              fi

              # Add ./target/debug/* to PATH
              export PATH="$PATH:$(pwd)/target/debug"

              # Add ./target/release/* to PATH
              export PATH="$PATH:$(pwd)/target/release"

              # Create symbolic link to movement repository
              mkdir -p .vendors
              ln -sfn ${movementRepo} .vendors/movement

              # Copy over ./githooks/pre-commit to .git/hooks/pre-commit
              cp $(pwd)/.githooks/pre-commit $(pwd)/.git/hooks/pre-commit
              chmod +x $(pwd)/.git/hooks/pre-commit

              cat <<'EOF'
               MOVEMENT => MOVEMENT APTOS
              EOF

              echo "Migrates Movement to Movement Aptos."
            '';
          };
        };
      }
    );
}
