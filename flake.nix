{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
      };
    in
    {
      packages.x86_64-linux.default = pkgs.rustPlatform.buildRustPackage {
        pname = "firefly-iii-importer";
        version = "0.1.1";
	useFetchCargoVendor = true;

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        buildInputs = with pkgs; [
          openssl
        ];

        src = ./.;
        cargoHash = "sha256-5eWfRIyvNaZiQchOv3EONxyP5oJwHDPjkxHQym0z/3Q=";
      };

      devShells.x86_64-linux.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          cargo
          nixd
          openssl
          pkg-config
          rust-analyzer
          rustc
        ];
      };
    };
}
