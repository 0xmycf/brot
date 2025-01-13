{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.11";
  };
  outputs = {
    self,
    nixpkgs,
    utils,
  }:
    utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};

        brot = pkgs.rustPlatform.buildRustPackage {
          pname = "brot";
          version = "1.0.0";
          cargoLock.lockFile = ./Cargo.lock;
          src = pkgs.lib.cleanSource ./.;
        };
      in {
        packages = {
          brot = brot;
          default = brot;
        };

        devShell = pkgs.mkShell {
          buildInputs = with pkgs; [
            # stuff i use inside rust / cargo
            rustc
            cargo
            rustfmt
            # linter
            clippy
            # lsp stuff
            rust-analyzer
          ];
        };
      }
    );
}
