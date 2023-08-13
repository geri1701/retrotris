{
  description = "rust piston development";
  inputs = {
    nixpkgs.url      = "github:nixos/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      with nixpkgs;
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.stable."1.70.0".default.override {
          extensions = [
            "clippy-preview"
            "rust-src"
            "rustfmt-preview"
            "rust-analysis"
          ];
        };
      in
      {
        devShell = pkgs.mkShell rec {
          buildInputs = [
            rust
            pkgs.rustup
            pkgs.cmake
            pkgs.pkg-config
            pkgs.python39
            pkgs.alsa-lib
            pkgs.libGL
            pkgs.xorg.libX11
            pkgs.xorg.libXcursor
            pkgs.xorg.libXrandr
            pkgs.xorg.libXi
            pkgs.vulkan-tools
            pkgs.vulkan-headers
            pkgs.vulkan-loader
            pkgs.vulkan-validation-layers
            pkgs.bacon
            pkgs.cargo-show-asm
          ];

          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
        };
      }
    );
}