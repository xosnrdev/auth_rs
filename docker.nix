{ pkgs }:

let app = import ./default.nix { inherit pkgs; };
in pkgs.dockerTools.buildImage {
  name = "xosnrdev/${app.name}";
  tag = app.version;
  created = "now";

  config = {
    Env =
      [ "RUST_LOG=info" "APP__SERVER__HOST=0.0.0.0" "APP__SERVER__PORT=8080" ];
    Cmd = [ "${app}/bin/${app.pname}" ];
    Labels = {
      "org.opencontainers.image.title" = app.pname;
      "org.opencontainers.image.version" = app.version;
      "org.opencontainers.image.description" = app.meta.description;
      "org.opencontainers.image.documentation" = app.meta.homepage;
      "org.opencontainers.image.authors" = "xosnrdev";
      "org.opencontainers.image.source" = app.meta.repository;
      "org.opencontainers.image.licenses" = "MIT";
    };
  };
}
