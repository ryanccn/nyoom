{
  description = "A small CLI Firefox userchrome manager";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { self
    , nixpkgs
    , rust-overlay
    }:
    let
      version = builtins.substring 0 8 self.lastModifiedDate or "dirty";

      inherit (nixpkgs) lib;

      mkSystems = sys: builtins.map (arch: "${arch}-${sys}") [ "x86_64" "aarch64" ];
      systems = mkSystems "linux" ++ mkSystems "darwin";

      forAllSystems = lib.genAttrs systems;

      nixpkgsFor = forAllSystems (system:
        import nixpkgs {
          inherit system;
          overlays = [ self.overlays.default rust-overlay.overlays.default ];
        });

      forEachSystem = fn: forAllSystems (system:
        fn {
          inherit system;
          pkgs = nixpkgsFor.${system};
        });

      toolchainFor = forEachSystem (p: p.pkgs.rust-bin.stable.latest.default);
    in
    {
      checks = forEachSystem ({ pkgs, system }:
        let
          formatter = self.formatter.${system};
        in
        {
          fmt = pkgs.runCommand "check-fmt" { }
            ''
              ${pkgs.lib.getExe formatter} --check ${self}
              touch $out
            '';
        });

      devShells = forEachSystem ({ pkgs, system }:
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              rust-analyzer
              toolchainFor.${system}
            ];

            RUST_BACKTRACE = 1;
            RUST_SRC_PATH = "${toolchainFor.${system}}/lib/rustlib/src/rust/library";
          };
        });

      formatter = forEachSystem (p: p.pkgs.nixpkgs-fmt);

      packages = forEachSystem ({ pkgs, ... }: {
        inherit (pkgs) nyoom;
        default = pkgs.nyoom;
      });

      overlays.default = _: prev: {
        nyoom = prev.callPackage
          ({ darwin, lib, lto ? true, optimizeSize ? true, pkg-config, rustPlatform, stdenv, self, version }:
            rustPlatform.buildRustPackage
              {
                pname = "nyoom";
                inherit version;

                src = self;

                cargoLock.lockFile = "${self}/Cargo.lock";
                RUSTFLAGS = ""
                  + lib.optionalString lto " -C lto=thin -C embed-bitcode=yes"
                  + lib.optionalString optimizeSize " -C codegen-units=1 -C strip=symbols -C opt-level=z";

                buildInputs = [ ]
                  ++ lib.optionals stdenv.isDarwin (with darwin.apple_sdk.frameworks; [
                  CoreServices
                  Security
                ]);
                nativeBuildInputs = [ pkg-config ];
              })
          { inherit self version; };
      };
    };
}
