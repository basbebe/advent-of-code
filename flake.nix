{
  description = "Advent of Code";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.treefmt-nix.flakeModule ];

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
      perSystem =
        {
          config,
          self',
          inputs',
          pkgs,
          system,
          ...
        }:
        let
          pkgs = import inputs.nixpkgs {
            inherit system;
            overlays = [
              (import inputs.rust-overlay)
            ];
          };

          rust-toolchain = pkgs.rust-bin.stable."1.83.0".default.override {
            extensions = [ "rust-analyzer" ];
          };
        in
        {
          _module.args.pkgs = pkgs;

          devShells = rec {
            rust = pkgs.mkShell {
              name = "rust-shell";
              packages = with pkgs; [
                rust-toolchain
                config.treefmt.build.wrapper
              ];
            };
            default = rust;
          };

          treefmt = {
            programs.deno.enable = true;
            programs.nixfmt.enable = true;
            programs.nixfmt.package = pkgs.nixfmt-rfc-style;
            programs.rustfmt.enable = true;
            programs.taplo.enable = true;
          };
        };
      flake = {
        # The usual flake attributes can be defined here, including system-
        # agnostic ones like nixosModule and system-enumerating ones, although
        # those are more easily expressed in perSystem.

      };
    };
}
