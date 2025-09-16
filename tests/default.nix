{ pkgs, lib, uv2nix, pyproject-nix, pyproject-build-systems, openqasm-parser }:
let
  workspace = uv2nix.lib.workspace.loadWorkspace { workspaceRoot = ./env; };
  python = pkgs.python312;

  overlay = workspace.mkPyprojectOverlay {
    sourcePreference = "wheel";
  };

  pythonSet =
    (pkgs.callPackage pyproject-nix.build.packages {
      inherit python;
    }).overrideScope
      (
        lib.composeManyExtensions [
          pyproject-build-systems.overlays.default
          overlay
        ]
      );

  virtualenv = pythonSet.mkVirtualEnv "test-env" workspace.deps.default;
in
{
  packages = {
    test-env = virtualenv;

    check = pkgs.stdenv.mkDerivation {
      pname = "check";
      version = "0.1.0";
      src = ./testsuite;

      buildInputs = [
        virtualenv
        openqasm-parser
      ];

      buildPhase = ''
        mkdir -p "$out/bin"
        cp -r $src "$out/tests"
        cat <<EOF >|"$out/bin/check"
        #!/bin/sh
        cd $out
        ${virtualenv}/bin/pytest -p no:cacheprovider ./tests --binary ${openqasm-parser}/bin/openqasm-parser -vvv
        EOF
        chmod +x "$out/bin/check"
        patchShebangs "$out/bin/check"
      '';
    };
  };

  devShells = {
    test-env = pkgs.mkShell {
      packages = [
        virtualenv
        pkgs.uv
      ];

      env = {
        UV_PYTHON = python.interpreter;
        UV_PYTHON_DOWNLOADS = "never";
      };

      shellHook = ''
        unset PYTHONPATH
      '';
    };
  };
}
