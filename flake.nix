let
  rustVersion = "1.61.0";
  cargo2nixVersion = "release-0.11.0";
in
{
  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/${cargo2nixVersion}";
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.follows = "cargo2nix/nixpkgs";
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [cargo2nix.overlays.default];
        };

        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "${rustVersion}";
          packageFun = import ./Cargo.nix;
        };

      in rec {
        packages = {
          # replace hello-world with your package name
          testability-linter = (rustPkgs.workspace.testability-linter {}).bin;
          default = packages.testability-linter;
        };
      }
    );
}
