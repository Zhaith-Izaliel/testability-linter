{
  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    flake-utils.follows = "cargo2nix/flake-utils";
    nixpkgs.follows = "cargo2nix/nixpkgs";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = inputs: with inputs;
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [cargo2nix.overlays.default];
        };


        # 2. Builds the rust package set, which contains all crates in your cargo workspace's dependency graph.
        # `makePackageSet` accepts the following arguments:
        # - `packageFun` (required): The generated `Cargo.nix` file, which returns the whole dependency graph.
        # - `workspaceSrc` (optional): Sources for the workspace can be provided or default to the current directory.
        # You must set some combination of `rustChannel` + `rustVersion` or `rustToolchain`.
        # - `rustToolchain` (optional): Completely override the toolchain.  Must provide rustc, cargo, rust-std, and rust-src components
        # - `rustChannel` (optional): "nightly" "stable" "beta".  To support legacy use, this can be a version when supplied alone.  If unspecified, defaults to "stable".
        # - `rustVersion` (optional): "1.60.0" "2020-12-30".  If not supplied, "latest" will be assumed.
        # - `rustProfile` (optional): "minimal" or "default" usually.  "minimal" if not specified (for faster builds)
        # - `extraRustComponents` (optional): ["rustfmt" "clippy"].
        # - `packageOverrides` (optional):
        #     A function taking a package set and returning a list of overrides.
        #     Overrides are introduced to provide native inputs to build the crates generated in `Cargo.nix`.
        #     See `overlay/lib/overrides.nix` on how to create overrides and `overlay/overrides.nix` for a list of predefined overrides.
        #     Most of the time, you can just use `overrides.all`. You can hand-pick overrides later if your build becomes too slow.
        # - `rootFeatures` (optional):
        #     A list of activated features on your workspace's crates.
        #     Each feature should be of the form `<crate_name>[/<feature>]`.
        #     If `/<feature>` is omitted, the crate is activated with no default features.
        #     The default behavior is to activate all crates with default features.
        # - `fetchCrateAlternativeRegistry` (optional): A fetcher for crates on alternative registries.
        # - `release` (optional): Whether to enable release mode (equivalent to `cargo build --release`), defaults to `true`.
        # - `hostPlatformCpu` (optional):
        #     Equivalent to rust's target-cpu codegen option. If specified "-Ctarget-cpu=<value>" will be added to the set of rust
        #     flags used for compilation of the package set.
        # - `hostPlatformFeatures` (optional):
        #     Equivalent to rust's target-feature codegen option. If specified "-Ctarget-feature=<values>" will be added to the set of rust
        #     flags used for compilation of the package set. The value should be a list of the features to be turned on, without the leading "+",
        #     e.g. `[ "aes" "sse2" "ssse3" "sse4.1" ]`.  They will be prefixed with a "+", and comma delimited before passing through to rust.
        #     Crates that check for CPU features such as the `aes` crate will be evaluated against this argument.
        #    rustcLinkFlags (optional):
        #     Pass extra flags directly to rustc during non-build invocations
        #    rustcBuildFlags (optional):
        #     Pass extra flags directly to Rustc during build invocations
        # - `target` (optional):
        #     Set an explicit Rust output target.  Overrides the translation
        #     from Nix targets to Rust targets.  See overlay/lib/rust-triple.nix
        #     for more info.
        rustPkgs = pkgs.rustBuilder.makePackageSet {
          rustVersion = "1.61.0";
          packageFun = import ./Cargo.nix;
        };

        cargo2nixBin = cargo2nix.packages."${system}".cargo2nix;

        workspaceShell = (rustPkgs.workspaceShell {
          packages = [
            pkgs.rust-analyzer
            cargo2nixBin
          ];
          # shellHook = ''
          #   export PS1="\033[0;31m☠dev-shell☠ $ \033[0m"
          # '';
        }); # supports override & overrideAttrs
      in rec {
        devShells = {
          # nix develop
          default = workspaceShell;
        };
        packages = {
          # replace hello-world with your package name
          testability-linter = (rustPkgs.workspace.testability-linter {}).bin;
          default = packages.testability-linter;
          shell = devShells.default;
        };
      }
    );
}
