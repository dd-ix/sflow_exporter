{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = (import nixpkgs) {
            inherit system;
          };
        in
        {
          packages = rec {
            sflow-exporter = pkgs.callPackage ./derivation.nix {
              cargoToml = ./Cargo.toml;
            };
            default = sflow-exporter;
          };
        }
      ) // {
      overlays.default = _: prev: {
        sflow-exporter = self.packages."${prev.system}".default;
      };

      nixosModules = rec {
        sflow-exporter = import ./module.nix;
        default = sflow-exporter;
      };
    };
}
