{
  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {inherit system;};
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = [
        pkgs.google-cloud-sdk
        pkgs.rustc
        pkgs.rustfmt
        pkgs.cargo
      ];
    };
  };
}
