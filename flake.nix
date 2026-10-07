{
  description = "UTD Devday shell";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.systems.url = "github:nix-systems/default";
  inputs.flake-utils = {
    url = "github:numtide/flake-utils";
    inputs.systems.follows = "systems";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };
      in {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            # Claude did this idk

            #            (pkgs.python3.override {
            #              packageOverrides = import ./python-packages.nix;
            #            }).withPackages
            #            (ps:
            #              with ps; [
            #                certifi
            #                charset-normalizer
            #                customtkinter
            #                darkdetect
            #                idna
            #                pyserial
            #                requests
            #                urllib3
            #                prompt_toolkit
            #              ])

            # Claude did this idk

            (pkgs.python3.withPackages (ps:
              with ps; [
                pip
                requests
                certifi
                charset-normalizer
                customtkinter
                darkdetect
                idna
                pyserial
                requests
                urllib3
                prompt-toolkit
              ]))
          ];
        };
        shellHook = ''
          export PS1='$\{VIRTUAL_ENV_PROMPT:+($VIRTUAL_ENV_PROMPT)}[\w]\n\$'
        '';
      }
    );
}
