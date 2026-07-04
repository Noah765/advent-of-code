{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    treefmt.url = "github:numtide/treefmt-nix";
    treefmt.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = inputs: let
    inherit (inputs.nixpkgs.lib) genAttrs getExe makeBinPath systems;
    eachSystem = f: genAttrs systems.flakeExposed (x: f inputs.nixpkgs.legacyPackages.${x});

    formatter = pkgs:
      (inputs.treefmt.lib.evalModule pkgs {
        programs.alejandra.enable = true;
        programs.rustfmt.enable = true;
      }).config.build.wrapper;
  in {
    packages = eachSystem (pkgs: rec {
      default = aoc;

      aoc =
        pkgs.writers.writeNuBin "aoc" {
          check = ''${getExe pkgs.nushell} --commands "nu-check --debug $out"'';
          makeWrapperArgs = ["--prefix PATH : ${makeBinPath [pkgs.cargo pkgs.nushell]}"];
        }
        ./aoc.nu;
    });

    devShells = eachSystem (pkgs: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          cargo
          clippy
          nushell
          rust-analyzer
          rustc
          inputs.self.packages.${pkgs.stdenv.system}.aoc
          (formatter pkgs)
        ];
      };
    });

    formatter = eachSystem formatter;
  };
}
