{inputs, ...}: {
  perSystem = {
    inputs',
    pkgs,
    self',
    ...
  }: {
    # NOTE import-from-derivation to patch flakes is still needed unfortunately https://github.com/NixOS/nix/issues/3920
    _module.args.pkgs = inputs.nix-flake-patch.lib.patchPkgs inputs'.nixpkgs.legacyPackages [inputs.patch] {overlays = [inputs.self.overlays.default];};
    packages = {
      # TODO only expose gauntlet-wayland on Linux
      # NOTE was getting infinite recursion using optional overlays and mkMerge here
      inherit (pkgs) gauntlet gauntlet-wayland;
      default = self'.packages.gauntlet;
    };
  };
}
