{
  description = "Rust dev environment with diesel_cli and ~/.cargo/bin in PATH";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rust = pkgs.rustup;
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [
            rust
            pkgs.pkg-config
            pkgs.libpq
          ];

          shellHook = ''
            export PATH="$HOME/.cargo/bin:$PATH"
            echo "🔧 Rust dev env ready. ~/.cargo/bin added to PATH."
          '';
        };
      });
}
