{
  description = "wasm-pack setup";

  inputs = {
    nixpkgs = { url = "github:nixos/nixpkgs/nixos-unstable"; };
    rust-overlay = { url = "github:oxalica/rust-overlay"; };
  };

  outputs = { nixpkgs, rust-overlay, ... }:
    let system = "x86_64-linux";
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

    in {
      devShell.${system} = let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlay];
        };
      in (({ pkgs, ... }:
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
nodePackages.http-server              libGL
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
          ];
          LD_LIBRARY_PATH = libPath;
          shellHook = "";
        }) { pkgs = pkgs; });
    };
}

