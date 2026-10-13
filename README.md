# Nix
This uses nix flake dev shells in order to manage dependencies & nixGL to run rviz2.
Download nixOS, enable flakes then run `nix develop`

## Nix-env
This project uses nix-env, if you don't have nix-env installed, you have to run:
source `<wherever your project is/install/setup.bash`

# Graphical Application

To run the simulation thing you need to run: 
`nix run --impure github:nix-community/nixGL -- ros2 launch roarm_moveit_cmd command_control.launch.py`
nix run --impure github:nix-community/nixGL -- 

# Note

In roarm_ws_em0/src/roarm_main/roarm_moveit_cmd/CMakeLists.txt I changed the boost thing. Left a comment. Just removed "system"
Also with moveit2_tutorials now
