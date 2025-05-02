{
  description = "A task management program";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {self, nixpkgs, flake-utils}: flake-utils.lib.eachDefaultSystem (system: let
    pkgs = import nixpkgs { inherit system; };
  in {
    packages = rec {
      nasin = pkgs.callPackage ./default.nix {};
      default = nasin;
    };
    devShells.default = pkgs.mkShell {
      buildInputs = with pkgs; [ gtk4 pkg-config libadwaita openssl ];
    };
  });
}
