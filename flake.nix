{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        craneLib = crane.mkLib pkgs;

        logs = craneLib.buildPackage ({
          pname = "logs";
          cargoExtraArgs = "-p logs";
          src = craneLib.cleanCargoSource ./.;
        });

        blinky = craneLib.buildPackage ({
          pname = "blinky";
          cargoExtraArgs = "-p blinky";
          src = craneLib.cleanCargoSource ./.;
        });

        button = craneLib.buildPackage ({
          pname = "button";
          cargoExtraArgs = "-p button";
          src = craneLib.cleanCargoSource ./.;
        });

        recieve = craneLib.buildPackage ({
          pname = "recieve";
          cargoExtraArgs = "-p recieve";
          src = craneLib.cleanCargoSource ./.;
        });

        send = craneLib.buildPackage ({
          pname = "send";
          cargoExtraArgs = "-p send";
          src = craneLib.cleanCargoSource ./.;
        });

        
      in
    {
      packages = {
        inherit logs blinky button recieve send;
      };
    });
}