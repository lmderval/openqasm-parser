{
  description = "A flake for the OpenQASM parser";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    systems.url = "github:nix-systems/x86_64-linux";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.systems.follows = "systems";
    };
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        localLib = import ./localLib { inherit pkgs; };
        project = import ./. { inherit pkgs localLib; };
      in
      {
        packages =
          project.packages //
          {
            default = project.packages.openqasm-parser;
          };

        devShells =
          project.devShells //
          {
            default = project.devShells.openqasm-parser;
          };

        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
