# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target ros2web_interfaces::ros2web_interfaces
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${ros2web_interfaces_TARGETS}.
if(ros2web_interfaces_TARGETS AND NOT TARGET ros2web_interfaces::ros2web_interfaces)
  add_library(ros2web_interfaces::ros2web_interfaces INTERFACE IMPORTED)
  set_target_properties(ros2web_interfaces::ros2web_interfaces PROPERTIES
    INTERFACE_LINK_LIBRARIES "${ros2web_interfaces_TARGETS}")
endif()
