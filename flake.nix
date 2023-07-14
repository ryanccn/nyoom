{
  description = "A small CLI Firefox userchrome manager";

  inputs.nixpkgs.url = "nixpkgs/nixos-unstable";

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
      devShells = forEachSystem ({ pkgs, ... }: {
        default = pkgs.mkShell {
          packages = [ pkgs.go ];
        };
      });

      formatter = forEachSystem (p: p.pkgs.nixpkgs-fmt);

      packages = forEachSystem ({ pkgs, ... }: {
        inherit (pkgs) nyoom;
        default = pkgs.nyoom;
      });

      overlays.default = _: prev: {
        nyoom = prev.buildGoModule rec {
          pname = "nyoom";
          inherit version;

          src = self;

          vendorHash = "sha256-4Wke/nlkF+NP+dZpPdXb35YfPk5Jsn7Oauyb4iitnGk=";

          meta = with prev.lib; {
            description = "A small CLI Firefox userchrome manager";
            homepage = "https://github.com/ryanccn/${pname}";
            license = licenses.gpl3;
            maintainers = [
              {
                name = "Ryan Cao";
                email = "hello@ryanccn.dev";
              }
            ];
          };
        };
      };
    };
}
