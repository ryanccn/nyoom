# SPDX-FileCopyrightText: 2024 Ryan Cao <hello@ryanccn.dev>
#
# SPDX-License-Identifier: GPL-3.0-or-later

{
  lib,
  stdenv,
  rustPlatform,
  installShellFiles,
  nix-filter,
  pkg-config,
  self,
  enableLTO ? true,
  enableOptimizeSize ? false,
}:
let
  year = builtins.substring 0 4 self.lastModifiedDate;
  month = builtins.substring 4 2 self.lastModifiedDate;
  day = builtins.substring 6 2 self.lastModifiedDate;
in
rustPlatform.buildRustPackage rec {
  pname = passthru.cargoToml.package.name;
  version = "${passthru.cargoToml.package.version}-unstable-${year}-${month}-${day}";

  src = nix-filter.lib.filter {
    root = self;
    include = [
      "src"
      "presets"
      "Cargo.lock"
      "Cargo.toml"
    ];
  };

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  nativeBuildInputs =
    [
      installShellFiles
    ]
    ++ lib.optionals stdenv.hostPlatform.isDarwin [
      pkg-config
    ];

  postInstall = lib.optionalString (stdenv.buildPlatform.canExecute stdenv.hostPlatform) ''
    installShellCompletion --cmd "${pname}" \
      --bash <("$out/bin/${pname}" completions bash) \
      --zsh <("$out/bin/${pname}" completions zsh) \
      --fish <("$out/bin/${pname}" completions fish)
  '';

  env =
    lib.optionalAttrs enableLTO {
      CARGO_PROFILE_RELEASE_LTO = "fat";
      CARGO_PROFILE_RELEASE_CODEGEN_UNITS = "1";
    }
    // lib.optionalAttrs enableOptimizeSize {
      CARGO_PROFILE_RELEASE_OPT_LEVEL = "z";
      CARGO_PROFILE_RELEASE_PANIC = "abort";
      CARGO_PROFILE_RELEASE_CODEGEN_UNITS = "1";
      CARGO_PROFILE_RELEASE_STRIP = "symbols";
    };

  doCheck = false;

  passthru = {
    cargoToml = lib.importTOML ../Cargo.toml;
  };

  meta = with lib; {
    description = "Small CLI Firefox userchrome manager";
    maintainers = with maintainers; [ ryanccn ];
    license = licenses.gpl3Plus;
    mainProgram = "nyoom";
  };
}
