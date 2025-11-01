{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = {
    self,
    nixpkgs,
    ...
  }: let
    inherit (nixpkgs.lib) genAttrs getExe makeBinPath mapAttrs systems;
    forAllSystems = f: mapAttrs f (genAttrs systems.flakeExposed (x: nixpkgs.legacyPackages.${x}));
  in {
    devShells = forAllSystems (system: pkgs: {default = pkgs.mkShell {packages = [pkgs.alejandra pkgs.rustfmt self.packages.${system}.aoc];};});
    packages = forAllSystems (_: pkgs: rec {
      aoc =
        pkgs.writers.writeNuBin "aoc" {
          check = ''${getExe pkgs.nushell} --commands "nu-check --debug $out"'';
          makeWrapperArgs = ["--prefix PATH : ${makeBinPath [pkgs.cargo pkgs.nushell]}"];
        }
        ./aoc.nu;
      default = aoc;
    });
    formatter = forAllSystems (_: pkgs: pkgs.alejandra);
  };
}
