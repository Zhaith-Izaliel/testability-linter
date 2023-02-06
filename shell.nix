# IMPORTANT: Use it only with nix-env-selector extension with VSCODE. In a shell, use nix develop.
{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  # nativeBuildInputs is usually what you want -- tools you need to run
  nativeBuildInputs = with pkgs; [
    rust-analyzer
    rustup
  ];
}