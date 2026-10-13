# Install script for directory: /home/omar/Documents/UTD/RIDE/roarm_ws_em0/src/moveit2_tutorials/doc/examples/realtime_servo

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/home/omar/Documents/UTD/RIDE/roarm_ws_em0/install/moveit2_tutorials")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Release")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Install shared libraries without execute permission?
if(NOT DEFINED CMAKE_INSTALL_SO_NO_EXE)
  set(CMAKE_INSTALL_SO_NO_EXE "0")
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

# Set path to fallback-tool for dependency-resolution.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/nix/store/l5qkpzsr4gxvksh45b3nhxbkyr5cviar-gcc-wrapper-15.3.0/bin/objdump")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/moveit2_tutorials/servo_keyboard_input" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/moveit2_tutorials/servo_keyboard_input")
    file(RPATH_CHECK
         FILE "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/moveit2_tutorials/servo_keyboard_input"
         RPATH "")
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/moveit2_tutorials" TYPE EXECUTABLE FILES "/home/omar/Documents/UTD/RIDE/roarm_ws_em0/build/moveit2_tutorials/doc/examples/realtime_servo/servo_keyboard_input")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/moveit2_tutorials/servo_keyboard_input" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/moveit2_tutorials/servo_keyboard_input")
    file(RPATH_CHANGE
         FILE "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/moveit2_tutorials/servo_keyboard_input"
         OLD_RPATH "/home/omar/Documents/UTD/RIDE/roarm_ws_em0/install/moveit_servo/lib:/nix/store/y5ja99j8kbyscjpgfrk6cvq5vpamwj16-ros-humble-rviz-ogre-vendor-11.2.29-r1/opt/rviz_ogre_vendor/lib:/nix/store/dq5glld7n5nnq2qzqiam1fshyjbfivjg-ros-humble-image-transport-3.1.13-r1/lib:/nix/store/vh4zz766pz9snsj0lrqq0rkrc901mzal-ros-humble-ruckig-0.9.2-r1/lib:/nix/store/45mkycn8k5x2xz4pkg9x32lfikb9458z-ros-humble-urdfdom-3.0.2-r2/lib:/nix/store/zav01gzjnf2dhwsh059dn15d1pghr0h5-ros-humble-octomap-1.9.8-r1/lib:"
         NEW_RPATH "")
    if(CMAKE_INSTALL_DO_STRIP)
      execute_process(COMMAND "/nix/store/l5qkpzsr4gxvksh45b3nhxbkyr5cviar-gcc-wrapper-15.3.0/bin/strip" "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/moveit2_tutorials/servo_keyboard_input")
    endif()
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  include("/home/omar/Documents/UTD/RIDE/roarm_ws_em0/build/moveit2_tutorials/doc/examples/realtime_servo/CMakeFiles/servo_keyboard_input.dir/install-cxx-module-bmi-Release.cmake" OPTIONAL)
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/moveit2_tutorials/servo_cpp_interface_demo" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/moveit2_tutorials/servo_cpp_interface_demo")
    file(RPATH_CHECK
         FILE "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/moveit2_tutorials/servo_cpp_interface_demo"
         RPATH "")
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/moveit2_tutorials" TYPE EXECUTABLE FILES "/home/omar/Documents/UTD/RIDE/roarm_ws_em0/build/moveit2_tutorials/doc/examples/realtime_servo/servo_cpp_interface_demo")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/moveit2_tutorials/servo_cpp_interface_demo" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/moveit2_tutorials/servo_cpp_interface_demo")
    file(RPATH_CHANGE
         FILE "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/moveit2_tutorials/servo_cpp_interface_demo"
         OLD_RPATH "/home/omar/Documents/UTD/RIDE/roarm_ws_em0/install/moveit_servo/lib:/nix/store/y5ja99j8kbyscjpgfrk6cvq5vpamwj16-ros-humble-rviz-ogre-vendor-11.2.29-r1/opt/rviz_ogre_vendor/lib:/nix/store/dq5glld7n5nnq2qzqiam1fshyjbfivjg-ros-humble-image-transport-3.1.13-r1/lib:/nix/store/vh4zz766pz9snsj0lrqq0rkrc901mzal-ros-humble-ruckig-0.9.2-r1/lib:/nix/store/45mkycn8k5x2xz4pkg9x32lfikb9458z-ros-humble-urdfdom-3.0.2-r2/lib:/nix/store/zav01gzjnf2dhwsh059dn15d1pghr0h5-ros-humble-octomap-1.9.8-r1/lib:"
         NEW_RPATH "")
    if(CMAKE_INSTALL_DO_STRIP)
      execute_process(COMMAND "/nix/store/l5qkpzsr4gxvksh45b3nhxbkyr5cviar-gcc-wrapper-15.3.0/bin/strip" "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/moveit2_tutorials/servo_cpp_interface_demo")
    endif()
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  include("/home/omar/Documents/UTD/RIDE/roarm_ws_em0/build/moveit2_tutorials/doc/examples/realtime_servo/CMakeFiles/servo_cpp_interface_demo.dir/install-cxx-module-bmi-Release.cmake" OPTIONAL)
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/moveit2_tutorials" TYPE DIRECTORY FILES "/home/omar/Documents/UTD/RIDE/roarm_ws_em0/src/moveit2_tutorials/doc/examples/realtime_servo/launch")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/moveit2_tutorials" TYPE DIRECTORY FILES "/home/omar/Documents/UTD/RIDE/roarm_ws_em0/src/moveit2_tutorials/doc/examples/realtime_servo/config")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
if(CMAKE_INSTALL_LOCAL_ONLY)
  file(WRITE "/home/omar/Documents/UTD/RIDE/roarm_ws_em0/build/moveit2_tutorials/doc/examples/realtime_servo/install_local_manifest.txt"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
endif()
