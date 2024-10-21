{
  description = "Nix-flake development environment for my personal website, johndesilenc.io";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
    in
    with pkgs;
    {
      devShells.${system}.default = mkShell {
        buildInputs = [
          (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
          openssl
          pkg-config
        ];
      };
    };
}
