{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    crane.url = "github:ipetkov/crane";

    pre-commit-hooks-nix = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    crane,
    pre-commit-hooks-nix,
  }: let
    systems = [
      "x86_64-linux"
      "aarch64-linux"
    ];
  in
    utils.lib.eachSystem systems (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      craneLib = crane.mkLib pkgs;
      lib = pkgs.lib;

      jsonFilter = path: _type: builtins.match ".*json$" path != null;
      jsonOrCargo = path: type:
        (jsonFilter path type) || (craneLib.filterCargoSources path type);

      src = lib.cleanSourceWith {
        src = ./.;
        filter = jsonOrCargo;
        name = "source";
      };

      common-args = {
        inherit src;
        strictDeps = true;

        #buildInputs = [pkgs.udev];
        nativeBuildInputs = [pkgs.installShellFiles];

        postInstall = ''
          installShellCompletion --cmd ppd \
            --bash ./target/release/build/ppd-*/out/ppd.bash \
            --fish ./target/release/build/ppd-*/out/ppd.fish \
            --zsh ./target/release/build/ppd-*/out/_ppd
          installManPage ./target/release/build/ppd-*/out/ppd.1
        '';
      };

      cargoArtifacts = craneLib.buildDepsOnly common-args;

      ppd = craneLib.buildPackage (common-args
        // {
          inherit cargoArtifacts;
        });

      pre-commit-check = hooks:
        pre-commit-hooks-nix.lib.${system}.run {
          src = ./.;

          inherit hooks;
        };
    in rec {
      checks = {
        inherit ppd;

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

        pre-commit-check = pre-commit-check {
          alejandra.enable = true;
        };
      };
      packages.ppd = ppd;
      packages.default = packages.ppd;

      apps.ppd = utils.lib.mkApp {
        drv = packages.ppd;
      };
      apps.default = apps.ppd;

      formatter = pkgs.alejandra;

      devShells.default = let
        checks = pre-commit-check {
          alejandra.enable = true;
          rustfmt.enable = true;
          clippy.enable = true;
        };
      in
        craneLib.devShell {
          packages = with pkgs; [
            rustfmt
            clippy
            cargo-deny
            cargo-about
            termshot
            #pkg-config
            #udev
            cargo-flamegraph
          ];
          shellHook = ''
            ${checks.shellHook}
          '';
        };
    })
    // {
      hydraJobs = {
        inherit (self) checks packages devShells;
      };
    };
}
