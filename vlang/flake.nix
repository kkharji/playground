{
  description = "Vlang Playground";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.utils.url =
    "github:numtide/flake-utils?rev=3cecb5b042f7f209c56ffd8371b2711a290ec797";
  inputs.devshell.url =
    "github:numtide/devshell?rev=7033f64dd9ef8d9d8644c5030c73913351d2b660";

  outputs = { self, ... }@inputs:
    with inputs.utils.lib;
    eachDefaultSystem (system:
      let
        overlays = [ inputs.devshell.overlay ];
        pkgs = import inputs.nixpkgs { inherit system overlays; };
      in {
        packages = flattenTree { };
        devShell = pkgs.mkShell {
          packages = with pkgs; [ openssl ];
          # env = [ ];
        };
      });
}
