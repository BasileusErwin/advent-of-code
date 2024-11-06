{
  pkgs ? import <nixpkgs> { },
}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    ocaml
    opam
    dune_2
    ocamlPackages.ocamlformat
    ocamlPackages.ocaml-lsp
    ocamlPackages.merlin
  ];

  shellHook = ''
    opam init -y
    eval $(opam env)
  '';
}
