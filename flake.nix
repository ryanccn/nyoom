{
  description = "A small CLI Firefox userchrome manager";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-23.05";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-overlay.follows = "rust-overlay";
    };
  };

  outputs =
    { self
    , nixpkgs
    , crane
    , rust-overlay
    , ...
    }:
    let
      version = builtins.substring 0 8 self.lastModifiedDate;

      linux = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      systems =
        [
          "x86_64-darwin"
          "aarch64-darwin"
        ]
        ++ linux;

      inherit (nixpkgs) lib;

      forAllSystems = lib.genAttrs systems;
      nixpkgsFor = forAllSystems (system:
        import nixpkgs {
          inherit system;
          overlays = [ self.overlays.default rust-overlay.overlays.default ];
        });

      forEachSystem = fn:
        forAllSystems (system:
          fn {
            inherit system;
            pkgs = nixpkgsFor.${system};
          });

      toolchainFor = forEachSystem (
        { pkgs
        , system
        , ...
        }:
        pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ];
          targets = lib.optionals (builtins.elem system linux) [
            "x86_64-unknown-linux-musl"
            "aarch64-unknown-linux-musl"
          ];
        }
      );
    in
    {
      devShells = forEachSystem
        ({ pkgs
         , system
         , ...
         }: {
          default = pkgs.mkShell {
            packages = with pkgs; [
              toolchainFor.${system}
              rust-analyzer
            ];

            RUST_BACKTRACE = 1;
          };
        });

      formatter = forEachSystem (p: p.pkgs.nixpkgs-fmt);

      packages = forEachSystem
        ({ pkgs
         , system
         , ...
         }:
          lib.recursiveUpdate
            {
              inherit (pkgs) nyoom;
              default = pkgs.nyoom;
            }
            (
              lib.optionalAttrs (builtins.elem system linux) {
                nyoom-static =
                  let
                    arch =
                      if system == "x86_64-linux"
                      then "x86_64"
                      else if system == "aarch64-linux"
                      then "aarch64"
                      else "";

                    flags.CARGO_BUILD_TARGET = "${arch}-unknown-linux-musl";
                    flags."CARGO_TARGET_${lib.toUpper arch}_UNKNOWN_LINUX_MUSL_LINKER" =
                      let
                        inherit (pkgs.pkgsStatic.stdenv) cc;
                      in
                      "${cc}/bin/${cc.targetPrefix}cc";
                  in
                  pkgs.nyoom.overrideAttrs (_:
                    {
                      # optimize for size
                      CARGO_BUILD_RUSTFLAGS = "-C strip=symbols -C target-feature=+crt-static -C opt-level=z -C codegen-units=1";
                    }
                    // flags);
              }
            ));

      overlays.default = _: prev:
        let
          crane' = crane.mkLib prev;
        in
        {
          nyoom =
            let
              isLinux = builtins.elem prev.system linux;
              buildRustPackage =
                if isLinux
                then (crane'.overrideToolchain toolchainFor.${prev.system}).buildPackage
                else crane'.buildPackage;
            in
            buildRustPackage {
              pname = "nyoom";
              inherit version;
              src = self;
            };
        };
    };
}
