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

      packageFn = pkgs:
        let
          inherit (pkgs.lib) licenses maintainers;
        in
        {
          nyoom = pkgs.buildGoModule rec {
            pname = "nyoom";
            inherit version;

            src = builtins.path {
              name = "${pname}-src";
              path = ./.;
            };

            vendorHash = "sha256-3tO/+Mnvl/wpS7Ro3XDIVrlYTGVM680mcC15/7ON6qM=";

            meta = {
              description = "A small CLI Firefox userchrome manager";
              homepage = "https://github.com/ryanccn/${pname}";
              license = licenses.gpl3;
              maintainers = [{
                name = "Ryan Cao";
                email = "hello@ryanccn.dev";
              }];
            };
          };
        };

      forAllSystems = nixpkgs.lib.genAttrs systems;
      nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; });
    in
    {
      devShells = forAllSystems (s:
        let
          pkgs = nixpkgsFor.${s};
          inherit (pkgs) mkShell;
        in
        {
          default = mkShell {
            packages = [ pkgs.go ];
          };
        });

      packages = forAllSystems (s:
        let
          p = packageFn nixpkgsFor.${s};
        in
        p // { default = p.nyoom; });

      overlays.default = _: prev: (packageFn prev);
    };
}
