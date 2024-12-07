{
  description = "Development environment for d4-oxide";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    d4 = {
      url = "github:SoftVarE-Group/d4v2/mt-kahypar";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { nixpkgs, d4, ... }:
    let
      lib = nixpkgs.lib;

      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];
    in
    {
      formatter = lib.genAttrs systems (system: nixpkgs.legacyPackages.${system}.nixfmt-rfc-style);
      devShells = lib.genAttrs systems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          d4Pkgs = d4.packages.${system};
        in
        {
          default = pkgs.mkShell {
            nativeBuildInputs = [
              pkgs.cmake
              pkgs.pkg-config
            ];

            buildInputs = [
              pkgs.boost.dev
              pkgs.pkgsStatic.mpfr.dev
              pkgs.pkgsStatic.gmp.dev
              d4Pkgs.mt-kahypar.dev
            ];
          };
        }
      );
    };
}
