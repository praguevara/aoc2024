{
  description = "Simple rust flake with cargo-aoc";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnfree = true;
        };

        cargo-aoc = pkgs.rustPlatform.buildRustPackage rec {
          pname = "cargo-aoc";
          version = "0.3";

          src = pkgs.fetchFromGitHub {
            owner = "gobanos";
            repo = "cargo-aoc";
            rev = "v${version}";
            sha256 = "sha256-uYkq6TTIHB3beA4fjxnsSClRflZo9ol+UIzE9lq4pfg=";
          };

          cargoHash = "sha256-TT+DSxuAMBKODExigQY2xWWUZHXAyf0gigdt2201qy0=";

          meta = {
            description = "AOC helper for Rust";
            homepage = "https://github.com/owner-name/cargo-aoc";
          };
        };

        aoc-cli = pkgs.rustPlatform.buildRustPackage rec {
          pname = "aoc-cli";
          version = "0.12.0";

          src = pkgs.fetchFromGitHub {
            owner = "scarvalhojr";
            repo = "aoc-cli";
            rev = "main";
            sha256 = "sha256-UM/sb50hbifhiq1lWoIYuYrWyKos0uf/HuAv/1bv6B0=";
          };

          cargoHash = "sha256-158332li7flFCORYKz3Y0WsNi8ITJz2SYXDfVzcL54E=";

          nativeBuildInputs = [
            pkgs.openssl
            pkgs.pkg-config
          ];

          buildInputs = [ pkgs.openssl ];
        };

      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              openssl
              rust-analyzer
              pkg-config
              lldb
              rust-bin.nightly.latest.default
              cargo-aoc
              aoc-cli
            ];
          };
      }
    );
}
