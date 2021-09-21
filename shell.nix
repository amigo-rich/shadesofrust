with (import <nixpkgs> {});
mkShell {
  buildInputs = [
    cargo
    clippy
    rustc
    rustfmt
  ];
}
