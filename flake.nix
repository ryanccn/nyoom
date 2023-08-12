{
  description = "A small CLI Firefox userchrome manager";

  nixConfig = {
    extra-substituters = [
      "https://nix.ryanccn.dev/nyoom"
    ];
    extra-trusted-public-keys = [
      "nyoom:I3711Q+jJWqxuAiJljlmwi/89eFY5+AdrJgoIAEyS9o="
    ];
  };

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-overlay.follows = "rust-overlay";
      inputs.flake-utils.follows = "flake-utils";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    crane,
    ...
  }: let
    version = builtins.substring 0 8 self.lastModifiedDate or "dirty";

    inherit (nixpkgs) lib;

    mkSystems = sys: builtins.map (arch: "${arch}-${sys}") ["x86_64" "aarch64"];
    systems = mkSystems "linux" ++ mkSystems "darwin";

    forAllSystems = lib.genAttrs systems;

    nixpkgsFor = forAllSystems (system:
      import nixpkgs {
        inherit system;
        overlays = [self.overlays.default rust-overlay.overlays.default];
      });

    forEachSystem = fn:
      forAllSystems (system:
        fn {
          inherit system;
          pkgs = nixpkgsFor.${system};
        });

    toolchainFor = forEachSystem (p: p.pkgs.rust-bin.stable.latest.default);
  in {
    checks = forEachSystem ({
      pkgs,
      system,
    }: let
      formatter = self.formatter.${system};
    in {
      fmt =
        pkgs.runCommand "check-fmt" {}
        ''
          ${pkgs.lib.getExe formatter} --check ${self}
          touch $out
        '';
    });

    devShells = forEachSystem ({
      pkgs,
      system,
    }: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          rust-analyzer
          toolchainFor.${system}
        ];

        RUST_BACKTRACE = 1;
        RUST_SRC_PATH = "${toolchainFor.${system}}/lib/rustlib/src/rust/library";
      };
    });

    formatter = forEachSystem (p: p.pkgs.alejandra);

    packages = forEachSystem ({pkgs, ...}: {
      inherit (pkgs) nyoom;
      default = pkgs.nyoom;
    });

    overlays.default = _: prev: {
      nyoom =
        prev.callPackage
        ({
          libiconv,
          darwin,
          lib,
          lto ? true,
          optimizeSize ? true,
          pkg-config,
          rustPlatform,
          installShellFiles,
          stdenv,
          version,
          system,
          self,
        }:
          rustPlatform.buildRustPackage
          rec {
            pname = "nyoom";
            inherit version;

            src = crane.lib.${system}.cleanCargoSource ./.;
            cargoLock.lockFile = ./Cargo.lock;

            RUSTFLAGS =
              ""
              + lib.optionalString lto " -C lto=thin -C embed-bitcode=yes"
              + lib.optionalString optimizeSize " -C codegen-units=1 -C strip=symbols -C opt-level=z";

            buildInputs =
              []
              ++ lib.optionals stdenv.isDarwin [
                darwin.apple_sdk_11_0.frameworks.CoreFoundation
                darwin.apple_sdk_11_0.frameworks.Security
                darwin.IOKit
                libiconv
              ];

            nativeBuildInputs = [
              pkg-config
              installShellFiles
            ];

            postInstall = ''
              tmp="$TMPDIR/nyoom-nix-completions"
              mkdir -p "$tmp"

              "$out/bin/${pname}" completions bash > "$tmp/nyoom.bash"
              "$out/bin/${pname}" completions zsh > "$tmp/nyoom.zsh"
              "$out/bin/${pname}" completions fish > "$tmp/nyoom.fish"

              installShellCompletion "$tmp/nyoom.bash" "$tmp/nyoom.zsh" "$tmp/nyoom.fish"
            '';
          })
        {inherit self version;};
    };
  };
}
