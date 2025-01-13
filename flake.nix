{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url =  "github:nixos/nixpkgs/nixos-24.11";
  };
  outputs = { self, nixpkgs, utils }: utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          # stuff i use inside rust / cargo
          rustc cargo rustfmt 
          # linter
          clippy
          # lsp stuff
          rust-analyzer
        ];
      };
    }
  );
}
