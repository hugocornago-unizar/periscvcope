{
  description = "Rust solution for perscvcope lab.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux"];
      perSystem = {
        config,
        pkgs,
        system,
        ...
      }:
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs;
            [
							cargo
							rustc
							rust-analyzer
							clippy
							rustfmt
            ];
        };
      };
    };
}
