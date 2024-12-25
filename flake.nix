{
  description = "Auth Service PoC in Rust";

  inputs = {
    nixpkgs.url =
      "github:NixOS/nixpkgs?rev=de1864217bfa9b5845f465e771e0ecb48b30e02d";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = import nixpkgs { inherit system; };
      in {
        packages = {
          default = pkgs.callPackage ./default.nix { inherit pkgs; };
          docker = pkgs.callPackage ./docker.nix { inherit pkgs; };
        };

        devShells.default = pkgs.callPackage ./shell.nix { inherit pkgs; };

        formatter = pkgs.nixfmt-classic;
      });
}
