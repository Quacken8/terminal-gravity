let
  sources = import ./npins;
  pkgs = import sources.nixpkgs { };
in
pkgs.mkShell {
  packages = with pkgs; [
    cargo
    rustc
    rust-analyzer
    clippy
    rustfmt


    xorg.libX11
    pkg-config
  ];

}
