# SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
#
# SPDX-License-Identifier: GPL-3.0-or-later

{
  description = "Small CLI Firefox userchrome manager";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    nix-filter.url = "github:numtide/nix-filter";
  };

  outputs =
    {
      self,
      nixpkgs,
      nix-filter,
    }:
    let
      inherit (nixpkgs) lib;
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forAllSystems = lib.genAttrs systems;
      nixpkgsFor = forAllSystems (system: nixpkgs.legacyPackages.${system});
    in
    {
      checks = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};

          mkFlakeCheck =
            {
              name,
              nativeBuildInputs ? [ ],
              command,
              extraConfig ? { },
            }:
            pkgs.stdenv.mkDerivation (
              {
                name = "check-${name}";
                inherit (self.packages.${system}.nyoom) src;
                inherit nativeBuildInputs;

                strictDeps = true;

                buildPhase = ''
                  ${command}
                  touch "$out"
                '';

                doCheck = false;
                dontInstall = true;
                dontFixup = true;
              }
              // extraConfig
            );
        in
        {
          nixfmt = mkFlakeCheck {
            name = "nixfmt";
            nativeBuildInputs = with pkgs; [ nixfmt-rfc-style ];
            command = "nixfmt --check .";
          };

          rustfmt = mkFlakeCheck {
            name = "rustfmt";

            nativeBuildInputs = with pkgs; [
              cargo
              rustfmt
            ];

            command = "cargo fmt --check";
          };

          clippy = mkFlakeCheck {
            name = "clippy";

            nativeBuildInputs =
              with pkgs;
              [
                rustPlatform.cargoSetupHook
                cargo
                rustc
                clippy
                clippy-sarif
                sarif-fmt
              ]
              ++ lib.optionals stdenv.hostPlatform.isDarwin [
                pkg-config
              ];

            command = ''
              cargo clippy --all-features --all-targets --tests \
                --offline --message-format=json \
                | clippy-sarif | tee $out | sarif-fmt
            '';

            extraConfig = {
              inherit (self.packages.${system}.nyoom) cargoDeps buildInputs;
            };
          };

          reuse = mkFlakeCheck {
            name = "reuse";
            extraConfig = {
              src = self;
            };

            nativeBuildInputs = with pkgs; [
              reuse
            ];

            command = "reuse lint";
          };
        }
      );

      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              rustfmt
              clippy
              rust-analyzer

              reuse

              cargo-audit
              cargo-bloat
              cargo-expand

              libiconv
            ];

            inputsFrom = [ self.packages.${system}.nyoom ];

            env = {
              RUST_BACKTRACE = 1;
              RUST_SRC_PATH = toString pkgs.rustPlatform.rustLibSrc;
            };
          };
        }
      );

      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgsFor.${system};
          packages = self.overlays.default null pkgs;
        in
        {
          inherit (packages) nyoom;
          default = packages.nyoom;
        }
      );

      legacyPackages = forAllSystems (
        system: nixpkgsFor.${system}.callPackage ./nix/static.nix { inherit nix-filter self; }
      );

      overlays.default = _: prev: {
        nyoom = prev.callPackage ./nix/package.nix { inherit nix-filter self; };
      };

      formatter = forAllSystems (system: nixpkgsFor.${system}.nixfmt-rfc-style);
    };
}
