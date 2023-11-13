{
  description = "linc";
  inputs = {
    # https://status.nixos.org/
    nixpkgs.url = github:NixOS/nixpkgs/nixos-unstable;
    rust-overlay.url = github:oxalica/rust-overlay;
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      pkgs = import nixpkgs {
        overlays = [ rust-overlay.overlays.default ];
        system = "x86_64-linux";
      };
    in
    {
      formatter.x86_64-linux = pkgs.nixpkgs-fmt;
      defaultPackage.x86_64-linux =
        pkgs.stdenv.mkDerivation {
          name = "build";
          src = self;
          buildPhase = "echo fo";
        };
      devShell.x86_64-linux =
        pkgs.mkShell {
          buildInputs = [
            pkgs.cargo
            pkgs.protobuf
            pkgs.trunk
            pkgs.just
            pkgs.leptosfmt
            (pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
              extensions = [
                "clippy"
                "llvm-tools-preview"
                "rust-analyzer"
                "rust-src"
                "rustfmt"
              ];
              targets = [ 
                "wasm32-unknown-unknown" 
                "x86_64-unknown-linux-musl" 
                ];
            }))
          ];
        };
    };
}
