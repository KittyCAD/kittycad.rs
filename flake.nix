{
  description = "kittycad.rs development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    naersk,
  }: let
    overlays = [
      (import rust-overlay)
      (self: super: {
        rustToolchain = super.rust-bin.stable.latest.default.override {
          targets = ["wasm32-unknown-unknown"];
          extensions = ["rustfmt" "llvm-tools-preview" "rust-src"];
        };

        # stand-alone nightly formatter so we get the fancy unstable flags
        nightlyRustfmt = super.rust-bin.selectLatestNightlyWith (toolchain:
          toolchain.default.override {
            extensions = ["rustfmt"]; # just the formatter
          });
      })
    ];

    allSystems = [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ];

    forAllSystems = f:
      nixpkgs.lib.genAttrs allSystems (system:
        f {
          pkgs = import nixpkgs {
            inherit overlays system;
          };
          system = system;
        });
  in {
    devShells = forAllSystems ({pkgs, ...}: {
      default = pkgs.mkShell {
        packages =
          (with pkgs; [
            rustToolchain
            nightlyRustfmt
            cargo-sort
            cargo-nextest
            toml-cli
            openssl
            postgresql
            pkg-config
          ])
          ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs; [
            ]);

        RUSTFMT = "${pkgs.nightlyRustfmt}/bin/rustfmt";
      };
    });

    packages = forAllSystems ({
      pkgs,
      system,
    }: let
      naersk-lib = pkgs.callPackage naersk {
        cargo = pkgs.rustToolchain;
        rustc = pkgs.rustToolchain;
      };
    in {
      zoo = naersk-lib.buildPackage {
        pname = "zoo";
        version = "0.1.0";
        release = true;
        src = ./.;

        buildInputs = [pkgs.openssl pkgs.pkg-config];
      };
      default = self.packages.${system}.zoo;
    });
  };
}
