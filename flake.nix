{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = {nixpkgs, ...}: let
    inherit (nixpkgs) lib legacyPackages;
    forAllSystems = lib.genAttrs lib.systems.flakeExposed;
    buildAoc = pkgs:
      pkgs.writers.writeNuBin "aoc" {
        check = ''${lib.getExe pkgs.nushell} --commands "nu-check --debug $out"'';
        makeWrapperArgs = ["--prefix PATH : ${lib.makeBinPath (with pkgs; [cargo nushell])}"];
      }
      ./aoc.nu;
  in {
    devShells = forAllSystems (x: {default = legacyPackages.${x}.mkShell {packages = with legacyPackages.${x}; [cargo clippy nushell rustc rustfmt (buildAoc legacyPackages.${x})];};});
    packages = forAllSystems (x: rec {
      aoc = buildAoc legacyPackages.${x};
      default = aoc;
    });
    formatter = forAllSystems (x: legacyPackages.${x}.alejandra);
  };
}
