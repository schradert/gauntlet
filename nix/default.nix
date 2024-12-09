{
  config,
  inputs,
  lib,
  withSystem,
  ...
}: {
  imports = [
    ./modules/home.nix
    ./modules/nixos.nix
    ./pkgs/devshell.nix
    ./pkgs/overlay.nix
    ./pkgs/packages.nix
  ];
  systems = import inputs.systems;
  # TODO why is this calling other architectures?! bug in flake-parts with patched nixpkgs?
  # perSystem = {pkgs, ...}: {formatter = pkgs.alejandra;};
  # NOTE the following is an adequate but messy workaround
  flake.formatter = lib.genAttrs config.systems (lib.flip withSystem ({pkgs, ...}: pkgs.alejandra));
}
