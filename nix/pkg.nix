{ pkgs, src }:

let
  version = "0.1.1";
in
pkgs.rustPlatform.buildRustPackage {
  pname = "conform";
  version = version;

  meta = {
    description = "Rust CLI app to validate YAML file using schemas.";
  };

  src = src;

  cargoLock = {
    lockFile = src + "/Cargo.lock";
  };
}
