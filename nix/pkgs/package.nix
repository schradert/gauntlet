{
  # Libraries
  lib,
  stdenv,
  # Builders
  buildPackages,
  fetchurl,
  importNpmLock,
  rustPlatform,
  runCommand,
  # Dependencies
  cctools,
  cmake,
  deno,
  libxkbcommon,
  nodejs,
  openssl,
  pkg-config,
  protobuf,
  # Parameters
  pname ? "gauntlet",
  version ? "v12-git",
  _src ? ../../.,
}: let
  inherit (lib) flatten getExe' optional sourceTypes;
  inherit (stdenv.hostPlatform) isDarwin isLinux rust system;
  src = _src;
  # Borrowed from other packages in nixpkgs https://github.com/search?q=repo%3ANixOS%2Fnixpkgs%20RUSTY_V8_ARCHIVE&type=code
  fetch_librusty_v8 = {
    version,
    hashes,
  }:
    fetchurl {
      name = "librusty_v8-${version}";
      url = "https://github.com/denoland/rusty_v8/releases/download/v${version}/librusty_v8_release_${rust.rustcTarget}.a";
      hash = hashes.${system};
      meta.version = version;
      meta.sourceProvenance = [sourceTypes.binaryNativeCode];
    };
in
  rustPlatform.buildRustPackage {
    inherit pname version src;
    npmDeps = importNpmLock {npmRoot = src;};
    useFetchCargoVendor = true;
    cargoHash = "sha256-3n1I/URokJdwhk8gOGhjeoQzcdIsfI70PLj0NRmyKo4=";
    env.RUSTY_V8_ARCHIVE = fetch_librusty_v8 {
      # This matches the resolved v8 library dependency in Cargo.lock
      version = "0.74.3";
      hashes = {
        x86_64-linux = "sha256-8pa8nqA6rbOSBVnp2Q8/IQqh/rfYQU57hMgwU9+iz4A=";
        aarch64-darwin = "sha256-Djnuc3l/jQKvBf1aej8LG5Ot2wPT0m5Zo1B24l1UHsM=";
        # TODO build on all supported architectures
        x86_64-darwin = "";
        aarch64-linux = "";
      };
    };
    # OPENSSL_CONFIG_DIR didn't work for vendored dependencies
    env.OPENSSL_NO_VENDOR = true;
    nativeBuildInputs = flatten [
      cmake
      pkg-config
      protobuf
      (optional isLinux libxkbcommon.dev)

      # Dependencies from buildNpmPackage
      nodejs
      nodejs.python
      importNpmLock.npmConfigHook
      (optional isDarwin cctools)
    ];
    buildInputs = [deno openssl];
    buildFeatures = ["release"];
    preBuild = "npm run build";
    meta.mainProgram = "gauntlet";
  }
