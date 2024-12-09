{self, ...}: {
  config,
  lib,
  pkgs,
  ...
}: let
  inherit (lib) mkEnableOption mkPackageOption;
in {
  options.programs.gauntlet = {
    enable = mkEnableOption "Gauntlet application launcher";
    package = mkPackageOption pkgs (
      if config.programs.gauntlet.wayland.enable
      then "gauntlet-wayland"
      else "gauntlet"
    ) {};
    service.enable = mkEnableOption "running Gauntlet as a service";
    wayland.enable = mkEnableOption "adding Wayland dependencies to executable";
  };
  config.nixpkgs.overlays = [self.overlays.default];
}
