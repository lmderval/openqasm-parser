{
  description = "A flake for the OpenQASM parser";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    systems.url = "github:nix-systems/x86_64-linux";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.systems.follows = "systems";
    };
    pyproject-nix = {
      url = "github:pyproject-nix/pyproject.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    uv2nix = {
      url = "github:pyproject-nix/uv2nix";
      inputs.pyproject-nix.follows = "pyproject-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    pyproject-build-systems = {
      url = "github:pyproject-nix/build-system-pkgs";
      inputs.pyproject-nix.follows = "pyproject-nix";
      inputs.uv2nix.follows = "uv2nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , uv2nix
    , pyproject-nix
    , pyproject-build-systems
    , ...
    }: (
      flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        localLib = import ./localLib { inherit pkgs; };
        project = import ./. { inherit pkgs localLib; };
        tests = import ./tests {
          inherit pkgs uv2nix pyproject-nix pyproject-build-systems;
          inherit (nixpkgs) lib;
          inherit (project.packages) openqasm-parser;
        };
      in
      {
        packages =
          project.packages //
          tests.packages //
          {
            default = project.packages.openqasm-parser;
          };

        devShells =
          project.devShells //
          tests.devShells //
          {
            default = project.devShells.openqasm-parser;
          };

        formatter = pkgs.nixpkgs-fmt;
      })
    );
}
