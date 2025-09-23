{ inputs, pkgs, ... }:

{
  languages.rust.enable = true;
  cachix.enable = false;
  packages = [
    pkgs.subxt
    # inputs.zombienet.packages.${pkgs.stdenv.system}.default
  ];
}
