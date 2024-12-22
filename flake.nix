{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
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
            sflow-exporter = pkgs.callPackage ./package.nix { };
            default = sflow-exporter;
          };
        }
      ) // {
      overlays.default = _: prev: {
        inherit (self.packages."${prev.system}") sflow-exporter;
      };

      nixosModules = rec {
        sflow-exporter = ./module.nix;
        default = sflow-exporter;
      };
    };
}
