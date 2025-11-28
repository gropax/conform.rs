{
  description = "Rust CLI project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages.default = pkgs.callPackage ./nix/pkg.nix {
          inherit pkgs;
          src = pkgs.lib.cleanSource ./.;  # Prevent rebuilding after modifying non source file
        };

        devShells.default = pkgs.mkShell {
          name = "conform";

          buildInputs = with pkgs; [
            rustc
            cargo
            rustfmt
            clippy
            rust-analyzer
          ];

          shellHook = ''
            rustup component add rust-src >/dev/null 2>&1 || true
          '';
        };

        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/conform";
        };
      }
    );
}
