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
          rustc cargo rustfmt cargo-modules
          rust-analyzer
        ];
      };
    }
  );
}
