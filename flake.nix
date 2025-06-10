{
  description = "rust development (retrotris)";
  inputs = {
    nixpkgs.url      = "github:nixos/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = {  nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      with nixpkgs;
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.nightly.latest.default.override {
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
            pkgs.python310
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
            pkgs.doxygen
            pkgs.xorg.libXft
            pkgs.glib
            pkgs.glib-networking
            pkgs.gobject-introspection
            pkgs.xorg.libXinerama
            pkgs.pango
            pkgs.cairo
          ];

          LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
        };
      }
    );
}
