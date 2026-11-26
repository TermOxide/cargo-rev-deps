{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      inherit (nixpkgs) lib;

      genSystems = lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];

      get-pkgs =
        system:
        import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

      eachSystem = f: genSystems (system: f (get-pkgs system));
    in
    {
      formatter = eachSystem (pkgs: pkgs.nixfmt-tree);

      devShells = eachSystem (
        pkgs:
        let
          inherit (pkgs.stdenv.hostPlatform) system;
          inherit (self.packages.${system}) rust-toolchain rustfmt-nightly;
        in
        {
          default = pkgs.mkShell {
            # rustfmt.toml uses unstable options
            env.RUSTFMT = lib.getExe' rustfmt-nightly "rustfmt";

            packages = [
              rust-toolchain
              rustfmt-nightly
              pkgs.cargo-workspaces
              self.formatter.${system}
            ];
          };
        }
      );

      packages = eachSystem (pkgs: {
        default = self.packages.${pkgs.stdenv.hostPlatform.system}.cargo-rev-deps;

        cargo-rev-deps = pkgs.callPackage ./package.nix { };

        rust-toolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "clippy"
            "rust-analyzer"
            "rust-src"
          ];
        };

        # rustfmt 1.9.0, the first with every option in rustfmt.toml. Keep in
        # sync with the format check in .github/workflows/ci.yml.
        rustfmt-nightly = pkgs.rust-bin.nightly."2026-03-01".minimal.override {
          extensions = [ "rustfmt" ];
        };
      });

      # `nix flake check` only evaluates `packages`, it builds `checks`.
      checks = eachSystem (pkgs: {
        inherit (self.packages.${pkgs.stdenv.hostPlatform.system}) cargo-rev-deps;
      });
    };
}
