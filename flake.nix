{
  inputs = {
    nix-ros-overlay.url = "github:lopsided98/nix-ros-overlay/master";
    nixpkgs.follows = "nix-ros-overlay/nixpkgs"; # IMPORTANT!!!
  };
  outputs = {
    self,
    nix-ros-overlay,
    nixpkgs,
  }:
    nix-ros-overlay.inputs.flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [nix-ros-overlay.overlays.default];
      };
      gl = pkgs.writeShellScriptBin "gl" ''
        exec nix run --impure github:nix-community/nixGL -- "$@"
      '';

      moveit-task-constructor-core-new =
        pkgs.rosPackages.humble.moveit-task-constructor-core.overrideAttrs
        (finalAttrs: previousAttrs: {
          postFixup =
            (previousAttrs.postFixup or "")
            + ''
              rm -f $out/share/pkgconfig/pybind11.pc
            '';
        });
    in {
      packages = {
        inherit moveit-task-constructor-core-new;
        default = moveit-task-constructor-core-new;
      };

      devShells.default = pkgs.mkShell {
        name = "ROS2 RIDE";
        packages = [
          gl
          #non ROS packages
          (pkgs.python3.withPackages (ps:
            with ps; [
              pip

              numpy
              aiohttp
              ujson
              aiortc
              av
              opencv-python
              pyserial
            ]))

          pkgs.blas
          pkgs.cmake
          pkgs.gdb
          pkgs.boost
          pkgs.colcon
          pkgs.net-tools
          (with pkgs.rosPackages.humble;
            buildEnv {
              underlay = true;
              paths = [
                #ROS packages
                ros-core
                controller-manager
                demo-nodes-cpp
                ament-cmake-core
                python-cmake-module
                joint-state-publisher-gui
                rviz2

                moveit-task-constructor-core-new

                #moveit
                moveit-core
                moveit-servo
                moveit-ros-planning-interface
                moveit-resources-panda-moveit-config
                moveit-ros-visualization
                #moveit-ros-perception
                moveit-resources-pr2-description
                moveit-visual-tools
                moveit-common

                foxglove-bridge
                ompl
                #               moveit-hybrid-planning
                #               moveit-kinematics
                #               moveit-msgs
                #               moveit-planners-ompl
                #               moveit-plugins
                #               moveit-py
                #               moveit-resources
                #               moveit-resources-fanuc-description
                #               moveit-resources-fanuc-moveit-config
              ];
            })
        ];
        shellHook = ''          #Not working??
                   alias gl="nix run --impure github:nix-community/nixGL --"
        '';
      };
    });
  nixConfig = {
    extra-substituters = ["https://ros.cachix.org"];
    extra-trusted-public-keys = ["ros.cachix.org-1:dSyZxI8geDCJrwgvCOHDoAfOm5sV1wCPjBkKL+38Rvo="];
  };
}
