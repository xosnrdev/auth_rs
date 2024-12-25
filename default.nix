{ pkgs }:

let
  manifest = pkgs.lib.importTOML ./Cargo.toml;
  package = manifest.package;
in pkgs.rustPlatform.buildRustPackage {
  pname = package.name;
  version = package.version;
  src = pkgs.lib.cleanSource ./.;
  cargoLock.lockFile = ./Cargo.lock;

  checkFlags = [
    # Skip impure tests
    "--skip=test_health_check"
    "--skip=test_user"
  ];

  checkType = "debug";

  meta = with pkgs.lib; {
    inherit (package) description homepage repository;
    license = licenses.mit;
    maintainers = [ maintainers.xosnrdev ];
  };
}
