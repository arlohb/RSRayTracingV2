{
  description = "A simple rust project";
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        name = "rs_ray_tracing_v2";
        version = "0.0.1";
        deps = with pkgs; [
          xorg.libX11
          xorg.libxcb
          xorg.libXcursor
          xorg.libXrandr
          xorg.libXi
          libGL

          nodejs
        ];

        # This will build the desktop version
        # For the web use the npm scripts
        package = pkgs.rustPlatform.buildRustPackage {
          inherit version;
          pname = name;

          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          # The desktop version doesn't need the nightly version like web does
          # It should work with nightly, but for some reason it doesn't
          nativeBuildInputs = deps ++ [ (pkgs.rust-bin.fromRustupToolchainFile ./stable-toolchain.toml) ];
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust-analyzer
            (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
          ] ++ deps;

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath deps;
        };

        packages = rec {
          "${name}" = package;
          default = package;
        };
      }
    );
}
