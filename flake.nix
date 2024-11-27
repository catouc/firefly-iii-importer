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
        version = "0.1.0";

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        buildInputs = with pkgs; [
          openssl
        ];

        src = ./.;
        cargoHash = "sha256-Ti6701CxBTWSYZPLA/JVojGqhz9PWqKWF1HFmj4zxak=";
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
