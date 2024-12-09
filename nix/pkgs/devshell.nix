{
  perSystem = {pkgs, ...}: {
    devShells.default = with pkgs;
      mkShell {
        packages = lib.flatten [
          deno
          nodejs
          cargo
          protobuf
          cmake
          (lib.optionals stdenv.hostPlatform.isLinux [
            libxkbcommon.dev
            gtk-layer-shell
          ])
        ];
      };
  };
}
