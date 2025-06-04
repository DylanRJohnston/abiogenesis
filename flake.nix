{
  inputs = {
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        dioxus-cli = final: prev: {
          dioxus-cli = prev.dioxus-cli.override (old: {
            rustPlatform = old.rustPlatform // {
              buildRustPackage =
                args:
                old.rustPlatform.buildRustPackage (
                  args
                  // {
                    src = old.fetchCrate {
                      pname = "dioxus-cli";
                      version = "0.7.0-alpha.1";
                      hash = "sha256-3b82XlxffgbtYbEYultQMzJRRwY/I36E1wgzrKoS8BU=";
                    };

                    cargoHash = "sha256-r42Z6paBVC2YTlUr4590dSA5RJJEjt5gfKWUl91N/ac=";
                    cargoPatches = [ ];
                    buildFeatures = [ ];
                  }
                );
            };
          });
        };

        overlays = [
          (import rust-overlay)
          dioxus-cli
        ];

        pkgs = import nixpkgs { inherit system overlays; };

        toolchain = pkgs.rust-bin.nightly.latest.complete.override {
          extensions = [ "rust-src" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        build-web = pkgs.writeShellScriptBin "build-web" ''
          set -o errexit
          set -o pipefail
          set -o nounset
          set -x

          cargo build --release --target wasm32-unknown-unknown --no-default-features
          wasm-bindgen ./target/wasm32-unknown-unknown/release/abiogenesis.wasm --no-typescript --out-name index --target web --out-dir ./site
          wasm-opt -all -Oz -o site/index_bg.reduced.wasm site/index_bg.wasm
          mv site/index_bg.reduced.wasm site/index_bg.wasm   
          zip -r site.zip site
        '';

        hot = pkgs.writeShellScriptBin "hot" ''
          set -o errexit
          set -o pipefail
          set -o nounset
          set -x

          dx serve --hot-patch --package abiogenesis --no-default-features --features hot_reload  
        '';
      in
      {
        packages.dioxus-cli = pkgs.dioxus-cli;

        devShell = pkgs.mkShell {
          buildInputs = [
            toolchain
            pkgs.iconv
            pkgs.wasm-bindgen-cli
            pkgs.cargo-watch
            pkgs.cargo-expand
            pkgs.dioxus-cli
            pkgs.simple-http-server
            build-web
            hot
          ];

          RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
        };
      }
    );
}
