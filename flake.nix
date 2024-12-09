{
  description = "https://github.com/project-gauntlet/gauntlet";
  outputs = inputs: inputs.flake-parts.lib.mkFlake {inherit inputs;} {imports = [./nix];};
  inputs = {
    nixpkgs.url = github:nixos/nixpkgs/nixos-unstable;
    systems.url = github:nix-systems/default;
    flake-parts.url = github:hercules-ci/flake-parts;
    flake-compat.url = github:edolstra/flake-compat;
    flake-compat.flake = false;

    # TODO track https://github.com/NixOS/nixpkgs/pull/282798 to bump nixpkgs and remove patching
    nix-flake-patch.url = github:schradert/nix-flake-patch;
    patch.url = https://github.com/NixOS/nixpkgs/pull/282798.patch;
    patch.flake = false;
  };
}
