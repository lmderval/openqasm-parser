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
      in
      {
        packages = {
          openqasm-parser = pkgs.rustPlatform.buildRustPackage {
            pname = "openqasm-parser";
            version = "0.1.0";
            src = self;

            cargoLock.lockFile = ./Cargo.lock;
          };

          default = self.packages.${system}.openqasm-parser;
        };

        devShells = {
          openqasm-parser = pkgs.mkShell {
            inputsFrom = [
              self.packages.${system}.openqasm-parser
            ];

            buildInputs = with pkgs; [
              rust-analyzer
              rustfmt
              clippy
            ];
          };

          default = self.devShells.${system}.openqasm-parser;
        };

        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
