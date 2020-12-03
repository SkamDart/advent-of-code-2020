with import <nixpkgs> {};

let
  crust = (rustChannels.nightly.rust.override { extensions = [ "rust-src" ]; });
in
stdenv.mkDerivation {
  name = "advent-of-code";
  buildInputs = [ crust rustracer protobuf ];
  RUST_SRC_PATH = "${crust}/lib/rustlib/src/rust/src";
}
