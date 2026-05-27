{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    crane.url = "github:ipetkov/crane";

    pre-commit-hooks-nix = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    nix-github-actions = {
      url = "github:nix-community/nix-github-actions";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    ci.url = "github:Lyndeno/ci";
    ci.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    crane,
    pre-commit-hooks-nix,
    nix-github-actions,
    ci,
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
          installManPage ./target/release/build/ppd-*/out/*.1
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

      test = pkgs.nixosTest {
        name = "ppd-test";
        nodes.machine = {
          config,
          pkgs,
          ...
        }: {
          services.power-profiles-daemon.enable = true;
          system.stateVersion = "24.11";
          environment.systemPackages = [ppd];
        };

        testScript = ''
          import sys
          machine.wait_for_unit("dbus.socket")
          ppctl_out = machine.execute("powerprofilesctl")
          print("powerprofilesctl:")
          print(ppctl_out[1])
          ppd_out = machine.execute("ppd")
          print("ppd:")
          print(ppd_out[1])
          if (ppd_out[1] == ppctl_out[1]):
            sys.exit(0)
          else:
            sys.exit(-1)
        '';
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

        # TODO: Make this x86 only
        #vm-test = test;

        hydra-spec = ci.lib.mkHydraCheck {
          inherit pkgs;
          specPackage = packages.hydra-spec;
          specFile = ./.hydra/spec.json;
        };

        mergify-check = ci.lib.mkMergifyCheck {
          inherit pkgs;
          mergifyPackage = packages.mergify;
          mergifyFile = ./.mergify.yml;
        };
      };
      packages.ppd = ppd;
      packages.default = packages.ppd;
      packages.hydra-spec = ci.lib.mkHydraSpec {
        inherit pkgs;
        owner = "Lyndeno";
        repo = "ppd-rs";
      };
      packages.mergify = ci.lib.mkMergifyConfig {
        inherit pkgs;
        projectName = "ppd-rs";
        checks = self.checks;
      };

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
      githubActions = nix-github-actions.lib.mkGithubMatrix {
        checks = {
          inherit (self.checks) x86_64-linux;
          # KVM is not working on arm runners
          aarch64-linux = builtins.removeAttrs self.checks.aarch64-linux ["vm-test"];
        };
      };
    };
}
