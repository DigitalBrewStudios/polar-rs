{
  description = "Description of the project";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    git-hooks-nix.inputs.nixpkgs.follows = "nixpkgs";
    git-hooks-nix.url = "github:cachix/git-hooks.nix";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    systems.url = "github:nix-systems/default";
    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.tar.gz";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [ inputs.git-hooks-nix.flakeModule ];
      perSystem =
        {
          config,
          system,
          pkgs,
          ...
        }:
        {
          _module.args.pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [ (import inputs.rust-overlay) ];
          };

          pre-commit = {
            check.enable = true;
            settings.hooks = {
              nixfmt-rfc-style.enable = true;
              flake-checker.enable = true;
              statix.enable = true;

              commitizen.enable = true;
              check-merge-conflicts.enable = true;
              no-commit-to-branch.enable = true;

              check-yaml.enable = true;
              yamlfmt.enable = true;
              yamllint.enable = true;

              "check-toml".enable = true;
              taplo.enable = true;
              rustfmt.enable = true;

              cargo-check.enable = true;
              clippy.enable = true;
              clippy.settings.extraArgs = builtins.concatStringsSep " " [
                "-Dwarnings"
              ];
            };
          };

          devShells.default = pkgs.mkShell {
            inherit (config.pre-commit) shellHook;
            packages =
              with pkgs;
              [
                rust-bin.stable.latest.default
                openssl
              ]
              ++ config.pre-commit.settings.enabledPackages # Pre-commit
              ++ lib.optionals stdenv.isDarwin [
                apple-sdk # Darwin
              ];

            RUST_BACKTRACE = "1";
          };
        };
    };
}
