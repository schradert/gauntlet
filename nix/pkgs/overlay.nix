{
  flake.overlays.default = final: _: {
    gauntlet-wayland = final.gauntlet.overrideAttrs (old: {
      buildInputs = old.buildInputs ++ (with final; [wayland wayland-protocols wlr-protocols]);
    });
    gauntlet = final.callPackage ./package.nix {
      # TODO convert tools from submodule to javascript dependency
      # Pull submodule from specific remote commit
      # TODO update package-lock.json
      # Seem to be version mismatches? also submodule package-lock.json doesn't seem to be resolved
      # TODO add deno types as a dependency rather than generating them on the fly
      # Nix sandbox prevents running network code (i.e. installing deno types from github URL)
      _src = let
        inherit (final) runCommand lib fetchFromGitHub fetchurl nodejs gnused;
        tools = fetchFromGitHub {
          owner = "project-gauntlet";
          repo = "tools";
          rev = "7bc5ef7d8326172b4353d37763b3c55e4ace051f";
          hash = "sha256-1JvHcqbIG6+Dp/CHeX/tOBPKuUpLBnGMWzrfYBZWSD8=";
        };
        deno-types = fetchurl {
          url = "https://github.com/denoland/deno/releases/download/v1.36.4/lib.deno.d.ts";
          hash = "sha256-faimw0TezsJVH8yYUJYS5BZ6FNJ3Ly2doru3AFuC68k=";
        };
      in
        runCommand "source" {} ''
          export PATH="${lib.makeBinPath (with final; [gnused jq moreutils])}:$PATH"
          mkdir -p tmp/{,tools,js/deno/dist}

          # Merge in submodule
          cp -R ${../../.}/* tmp
          cp -R ${tools}/* tmp/tools

          # Provide updated and consolidated lockfile
          cp -f ${./package-lock.json} tmp/package-lock.json

          # Implement @project-gauntlet/deno stub generation
          sed 's:/// <reference lib="deno\..*" />::g' ${deno-types} > tmp/js/deno/dist/lib.deno.d.ts
          jq '.scripts.["run-generator-source"] |= ""' tmp/js/deno/package.json | sponge tmp/js/deno/package.json

          # patch gauntlet build tool shebang
          jq ".scripts.build += \" && ${lib.getExe gnused} --in-place '1s:.*:#!${lib.getExe nodejs}:' ./bin/main.js && cat ./bin/main.js\"" tmp/tools/package.json | sponge tmp/tools/package.json

          cp -R tmp $out
        '';
    };
  };
}
