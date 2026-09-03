{ pkgs ? import <nixpkgs> {} }:
let
  rust-overlay = (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"));
  pkgs = (import <nixpkgs> {
    overlays = [ rust-overlay ];
  });
  rust = pkgs.rust-bin.stable.latest.default.override {
    extensions = [ "rust-src" ];
  };
in pkgs.mkShell {

  nativeBuildInputs = [
    pkgs.rust-analyzer
    pkgs.pkg-config
    pkgs.gcc
    pkgs.glfw
    #pkgs.glfw3
    pkgs.wayland
    pkgs.libx11
    pkgs.git
  ];


  #  # Requirements needed during compilation and execution
    buildInputs = with pkgs; [
      pkgs.libGL
      pkgs.glfw
      #pkgs.glfw3
    pkgs.wayland
    pkgs.libx11
    pkgs.git
    ];
#
  ##LD_LIBRARY_PATH = "${pkgs.stdenv.cc.cc.lib}/lib";
  ## Ensures compiled libraries can be found by Python extensions
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
    pkgs.wayland
    pkgs.glfw
    #pkgs.glfw3
    pkgs.glew
    pkgs.libGL
    pkgs.stdenv.cc.cc.lib
    pkgs.libx11
    pkgs.git
  ];

  packages = [
    rust
  ];
}
