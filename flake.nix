{

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.11-small";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, rust-overlay, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.stable.latest.default;
        publishScript = pkgs.writeShellScriptBin "crate-publish" ''
          cargo login $1
          cargo publish -p crossterm-keybind-core || echo "publish crossterm-keybind-core fail"
          sleep 10
          cargo publish -p crossterm-keybind-derive || echo "publish crossterm-keybind-derive fail"
          sleep 10
          cargo publish -p crossterm-keybind
        '';
        cargoTomlConfig = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      in
      with pkgs;
      {
        devShells = {
          default = mkShell {
            buildInputs = [
              rust
              publishScript
            ];
          };
          msrv = mkShell {
            buildInputs = [
              pkgs.rust-bin.stable.${cargoTomlConfig.workspace.package.rust-version}.minimal
            ];
          };
        }; 
      }
    );
}
