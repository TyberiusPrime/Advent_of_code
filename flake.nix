{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/24.05"; # that's 23.05
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    naersk,
    rust-overlay,
  }:
    utils.lib.eachDefaultSystem (system: let
      #pkgs = nixpkgs.legacyPackages."${system}";
      overlays = [(import rust-overlay) ];
      pkgs = import nixpkgs {inherit system overlays;
      };
      rust = pkgs.rust-bin.stable."1.81.0".default.override {
        targets = []; # additional targets this should read...
      };

      # Override the version used in naersk
      naersk-lib = naersk.lib."${system}".override {
        cargo = rust;
        rustc = rust;
      };

      bacon = pkgs.bacon;
    in rec {
      # `nix build`
      packages.advent_of_code = naersk-lib.buildPackage {
        pname = "advent_of_code";
        root = ./.;
        nativeBuildInputs = with pkgs; [pkg-config];
        buildInputs = with pkgs; [openssl cmake];
        release = true;
      };

      packages.check = naersk-lib.buildPackage {
        src = ./.;
        mode = "check";
        nativeBuildInputs = with pkgs; [];
        buildInputs = with pkgs; [];
      };
      packages.test = naersk-lib.buildPackage {
        src = ./.;
        mode = "test";
        nativeBuildInputs = with pkgs; [];
        buildInputs = with pkgs; [];
      };

      defaultPackage = packages.advent_of_code;

      # `nix run`
      apps.mbf-fastq-processor = utils.lib.mkApp {drv = packages.my-project;};
      defaultApp = apps.mbf-fastq-processor;

      # `nix develop`
      devShell = pkgs.mkShell {
        # supply the specific rust version
        nativeBuildInputs = [
          rust
          pkgs.rust-analyzer
          pkgs.git
          pkgs.cargo-udeps
          pkgs.cargo-crev
          pkgs.cargo-vet
          pkgs.cargo-outdated
          pkgs.cargo-audit
          pkgs.pkg-config
          pkgs.openssl
          pkgs.cargo-flamegraph
          pkgs.cmake
          bacon
        ];
      };
    });
}
# {

