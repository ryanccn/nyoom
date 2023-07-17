{
  description = "A small CLI Firefox userchrome manager";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };

  outputs =
    { self
    , nixpkgs
    , ...
    }:
    let
      version = builtins.substring 0 8 self.lastModifiedDate;

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forAllSystems = nixpkgs.lib.genAttrs systems;
      nixpkgsFor = forAllSystems (system:
        import nixpkgs {
          inherit system;
          overlays = [ self.overlays.default ];
        });

      forEachSystem = fn:
        forAllSystems (system:
          fn {
            inherit system;
            pkgs = nixpkgsFor.${system};
          });
    in
    {
      devShells = forEachSystem ({ pkgs, ... }:
        let
          inherit (pkgs) mkShell;
        in
        {
          default = mkShell {
            packages = with pkgs; [
              bash
            ];
          };
        });

      formatter = forEachSystem ({ pkgs, ... }: pkgs.nixpkgs-fmt);

      packages = forEachSystem ({ pkgs, ... }: {
        inherit (pkgs) nyoom;
        default = pkgs.nyoom;
      });

      overlays.default = _: prev: {
        nyoom = prev.pkgs.rustPlatform.buildRustPackage
          {
            pname = "nyoom";
            inherit version;

            src = self;

            cargoLock.lockFile = "${self}/Cargo.lock";

            buildInputs = with prev.pkgs; [ ];

            nativeBuildInputs = with prev.pkgs; [
              pkg-config
            ];
          };
      };
    };
}
