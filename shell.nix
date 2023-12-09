{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
  buildInputs = [
    wasm-pack
  ];
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}

