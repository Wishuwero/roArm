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
    in {
      devShells.default = pkgs.mkShell {
        name = "ROS2 RIDE";
        packages = [
          #non ROS packages
          (pkgs.python3.withPackages (ps:
            with ps; [
              pip

              aiohttp
              ujson
              aiortc
              av
              opencv-python
              pyserial
            ]))

          pkgs.blas
          pkgs.cmake
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

                #moveit
                moveit-core
                moveit-visual-tools
                moveit-common
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

                foxglove-bridge
                moveit-servo
              ];
            })
        ];
      };
    });
  nixConfig = {
    extra-substituters = ["https://ros.cachix.org"];
    extra-trusted-public-keys = ["ros.cachix.org-1:dSyZxI8geDCJrwgvCOHDoAfOm5sV1wCPjBkKL+38Rvo="];
  };
}
