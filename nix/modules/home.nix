flake: {
  flake.homeManagerModules.default = {
    config,
    lib,
    pkgs,
    ...
  }: let
    inherit (lib) elem getExe mkIf mkMerge mkOption platforms toList types;
    cfg = config.programs.gauntlet;
    toml = pkgs.formats.toml {};
  in {
    imports = [(import ./common.nix flake)];
    options.programs.gauntlet.config = mkOption {
      inherit (toml) type;
      default = {};
      description = "Application configuration in config.toml";
    };
    config = mkIf cfg.enable {
      assertions = toList {
        assertion = cfg.service.enable -> elem pkgs.stdenv.hostPlatform.system platforms.linux;
        message = "Running Gauntlet as a service is only currently supported on Linux";
      };
      home.packages = [cfg.package];
      xdg.configFile = mkMerge [
        (mkIf (cfg.service.enable) {"systemd/user/gauntlet.service".source = "${cfg.package}/lib/systemd/user/gauntlet.service";})
        (mkIf (cfg.config != {}) {"gauntlet/config.toml".source = toml.generate "gauntlet.config.toml" config.programs.gauntlet.config;})
      ];
    };
  };
}
