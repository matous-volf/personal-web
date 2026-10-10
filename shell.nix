let
  pkgs = import <nixpkgs> {
    overlays = [ (import rust-overlay) ];
  };
  rust-overlay = fetchGit {
    url = "https://github.com/oxalica/rust-overlay";
    rev = "e90fafa71b440fabcd52dd7fb017ab2da81c781a";
    ref = "master";
  };
  toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
in
pkgs.mkShell {
  packages = [
    toolchain
    pkgs.pkg-config
    pkgs.openssl
    pkgs.openssl.dev
    pkgs.perl
    pkgs.package-version-server
  ];
  env = {
    OPENSSL_NO_VENDOR = "1";
    RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
  };
}
