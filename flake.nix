{
  description = "wasm-pack setup";

  inputs = {
    nixpkgs = { url = "github:nixos/nixpkgs/nixos-unstable"; };
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = { url = "github:oxalica/rust-overlay"; };
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        libPath = with pkgs; lib.makeLibraryPath [
          libGL
          libxkbcommon
          wayland
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
        ];

      in
      {
        formatter = pkgs.nixpkgs-fmt;
        devShell =
          let
            pkgs = import nixpkgs {
              inherit system;
              overlays = [ rust-overlay.overlay ];
            };
          in
          (({ pkgs, ... }:
            pkgs.mkShell {
              buildInputs = with pkgs; [
                cmake
                mesa
                xorg.libX11
                xorg.libxcb
                libGL
                fontconfig
                cargo
                cargo-watch
                rust-analyzer
                nodejs
                wasm-pack
                pkg-config
                openssl
                wasm-pack
                nodePackages.http-server
                libGL
                libxkbcommon
                wayland
                xorg.libX11
                xorg.libXcursor
                xorg.libXi
                xorg.libXrandr

                (rust-bin.stable.latest.default.override {
                  extensions = [ "rust-src" ];
                  targets = [ "wasm32-unknown-unknown" ];
                })
              ]
              ++ (
                lib.optional stdenv.isDarwin [
                  darwin.apple_sdk.frameworks.Security
                  darwin.apple_sdk.frameworks.CoreServices
                  darwin.apple_sdk.frameworks.CoreFoundation
                  darwin.apple_sdk.frameworks.Foundation
                  darwin.apple_sdk.frameworks.AppKit
                  darwin.apple_sdk.frameworks.WebKit
                  darwin.apple_sdk.frameworks.Cocoa
                ]
              );

              LD_LIBRARY_PATH = libPath;
              shellHook = "";
            }) { pkgs = pkgs; });
      }
    );
}

