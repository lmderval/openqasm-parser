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
          openqasm-parser = pkgs.stdenv.mkDerivation {
            pname = "openqasm-parser";
            version = "0.1.0";
            src = self;

            buildInputs = with pkgs; [
              rustc
              cargo
            ];

            buildPhase = ''
              cargo build --release
            '';

            installPhase = ''
              mkdir -p $out/bin
              cp target/release/openqasm-parser $out/bin/openqasm-parser
            '';
          };

          default = self.packages.${system}.openqasm-parser;
        };

        formatter = pkgs.nixpkgs-fmt;
      }
    );
}
