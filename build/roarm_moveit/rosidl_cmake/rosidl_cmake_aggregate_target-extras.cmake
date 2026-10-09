# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target roarm_moveit::roarm_moveit
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${roarm_moveit_TARGETS}.
if(roarm_moveit_TARGETS AND NOT TARGET roarm_moveit::roarm_moveit)
  add_library(roarm_moveit::roarm_moveit INTERFACE IMPORTED)
  set_target_properties(roarm_moveit::roarm_moveit PROPERTIES
    INTERFACE_LINK_LIBRARIES "${roarm_moveit_TARGETS}")
endif()
