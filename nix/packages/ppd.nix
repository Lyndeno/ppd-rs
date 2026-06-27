{
  inputs,
  pkgs,
  ...
}: let
  craneLib = inputs.crane.mkLib pkgs;
  lib = pkgs.lib;

  jsonFilter = path: _type: builtins.match ".*json$" path != null;
  jsonOrCargo = path: type:
    (jsonFilter path type) || (craneLib.filterCargoSources path type);

  src = lib.cleanSourceWith {
    src = ./../../.;
    filter = jsonOrCargo;
    name = "source";
  };

  common-args = {
    inherit src;
    strictDeps = true;

    nativeBuildInputs = [pkgs.installShellFiles];

    postInstall = ''
      installShellCompletion --cmd ppd \
        --bash ./target/release/build/ppd-*/out/ppd.bash \
        --fish ./target/release/build/ppd-*/out/ppd.fish \
        --zsh ./target/release/build/ppd-*/out/_ppd
      installManPage ./target/release/build/ppd-*/out/*.1
    '';
  };

  cargoArtifacts = craneLib.buildDepsOnly common-args;

  ppd = craneLib.buildPackage (common-args
    // {
      inherit cargoArtifacts;

      passthru.tests = {
        ppd-clippy = craneLib.cargoClippy (common-args
          // {
            inherit cargoArtifacts;
            cargoClippyExtraArgs = "--all-targets -- --deny warnings";
          });

        ppd-fmt = craneLib.cargoFmt {
          inherit src;
        };

        ppd-deny = craneLib.cargoDeny {
          inherit src;
        };
      };
    });
in
  ppd
