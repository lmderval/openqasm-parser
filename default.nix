{ pkgs, localLib }:
let
  openqasm-parser = pkgs.rustPlatform.buildRustPackage {
    pname = "openqasm-parser";
    version = "0.2.0";
    src = localLib.filters.cleanSourceWithFiles {
      src = ./.;
      files = [
        "Cargo.toml"
        "Cargo.lock"
        "src/"
      ];
    };

    cargoLock.lockFile = ./Cargo.lock;
  };
in
{
  packages = {
    inherit openqasm-parser;
  };

  devShells = {
    openqasm-parser = pkgs.mkShell {
      inputsFrom = [
        openqasm-parser
      ];

      buildInputs = with pkgs; [
        rust-analyzer
        rustfmt
        clippy
      ];
    };
  };
}
