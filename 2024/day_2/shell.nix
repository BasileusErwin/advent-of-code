{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    clang
    cmake
    openssl
    pkg-config
    valgrind
  ];
}
