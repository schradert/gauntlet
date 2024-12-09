# Nix

The Nix package derivation is currently defined for all [default systems](https://github.com/nix-systems/default), and it can be integrated into NixOS and Home-Manager configurations as below.

## Installation

Here's how to reference the package derivation (and explicitly pin it) in your `flake.nix`:

``` nix
{
  inputs.gauntlet.url = github:project-gauntlet/gauntlet/<gauntlet_version_repository_tag>;
  inputs.gauntlet.inputs.nixpkgs.follows = "nixpkgs";
}
```

## Configuration

Under `programs.gauntlet`, the options provide the following:

1. `enable`: adds executable to system path
2. `service.enable`: runs daemon with systemd (MacOS launchd not yet supported)
3. `wayland.enable`: exposes wlroots layer-shell to packages for use in Wayland setups

### NixOS

``` nix
{inputs, ...}: {
  imports = [inputs.gauntlet.nixosModules.default];
  programs.gauntlet = {
    enable = true;
    service.enable = true;
    wayland.enable = true;
  };
}
```

### Home-Manager

Once `config.toml` is [supported](../README.md#application-config), Home-Manager can populate its contents with `programs.gauntlet.config`.

``` nix
{inputs, ...}: {
  imports = [inputs.gauntlet.homeManagerModules.default];
  programs.gauntlet = {
    enable = true;
    service.enable = true;
    wayland.enable = true;
    config = {};
  };
}
```

## Development

This implementation currently requires IFD to patch nixpkgs in order to support [duplicate rust dependencies from git](https://github.com/NixOS/nixpkgs/pull/282798), so nix commands that don't support it by default must specify `--allow-import-from-derivation`.

Because this project uses a submodule, the derivation is a little [unconventional](pkgs/overlay.nix). As you can see, there are also some build practices in use in this repository that create friction with the nix sandbox, so even more patching is required.

There are a few hashes required for this build:

1. `cargoHash`: there are git dependencies
2. `env.RUSTY_V8_ARCHIVE`: librusty_v8 builds take forever, so best practice is to fetch binaries (per system!)
3. `tools`: this repo is a submodule
4. `deno-types`: these type stubs are dynamic in the non-nix build script, so we need to pin them and override the script
