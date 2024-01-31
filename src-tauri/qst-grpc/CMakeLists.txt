cmake_minimum_required(VERSION 3.14)

project(qst-proto LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# if(NOT CMAKE_MODULE_PATH)
# set(CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/cmake)
# else()
# list(APPEND CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/cmake)
# endif()
set(qst_proto_path "${CMAKE_CURRENT_SOURCE_DIR}/src")

file(GLOB_RECURSE qst_proto_files "${qst_proto_path}/*.proto")

if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
  if(CMAKE_BUILD_TYPE STREQUAL "Release")
    set(CMAKE_PREFIX_PATH "C:/tools/release/grpc" ${CMAKE_PREFIX_PATH})
  else()
    set(CMAKE_PREFIX_PATH "C:/tools/debug/grpc" ${CMAKE_PREFIX_PATH})
  endif()
endif()

find_program(_PROTOBUF_PROTOC protoc)
find_program(_GRPC_CPP_PLUGIN_EXECUTABLE grpc_cpp_plugin)

set(qst_grpc_cpp_srcs "")
set(qst_grpc_cpp_hdrs "")

foreach(_proto_abs_path ${qst_proto_files})
  get_filename_component(_proto_name "${_proto_abs_path}" NAME_WE)
  set(qst_grpc_cpp_srcs
    ${qst_grpc_cpp_srcs}
    "${CMAKE_CURRENT_SOURCE_DIR}/cpp/${_proto_name}.pb.cc"
    "${CMAKE_CURRENT_SOURCE_DIR}/cpp/${_proto_name}.grpc.pb.cc")
  set(qst_grpc_cpp_hdrs
    ${qst_grpc_cpp_hdrs}
    "${CMAKE_CURRENT_SOURCE_DIR}/cpp/${_proto_name}.pb.h"
    "${CMAKE_CURRENT_SOURCE_DIR}/cpp/${_proto_name}.grpc.pb.h")
endforeach()

# message(STATUS "!!!${qst_grpc_cpp_files}")
set(qst_cpp_out_dir "${CMAKE_CURRENT_SOURCE_DIR}/cpp")

# message(STATUS "!!!${qst_cpp_out_dir}")
add_custom_command(
  OUTPUT ${qst_grpc_cpp_srcs} ${qst_grpc_cpp_hdrs}
  COMMAND ${CMAKE_COMMAND} -E make_directory "${qst_cpp_out_dir}"
  COMMAND ${_PROTOBUF_PROTOC}
  ARGS --grpc_out ${qst_cpp_out_dir}
  --cpp_out ${qst_cpp_out_dir}
  -I ${qst_proto_path}
  --plugin=protoc-gen-grpc=${_GRPC_CPP_PLUGIN_EXECUTABLE}
  ${qst_proto_files}
  DEPENDS ${qst_proto_files}
  VERBATIM
)
set(qst_grpc_go_srcs "")
foreach(_proto_abs_path ${qst_proto_files})
  get_filename_component(_proto_name "${_proto_abs_path}" NAME_WE)
  set(qst_grpc_go_srcs
    ${qst_grpc_go_srcs}
    "${CMAKE_CURRENT_SOURCE_DIR}/go/${_proto_name}.pb.go"
    "${CMAKE_CURRENT_SOURCE_DIR}/go/${_proto_name}.grpc.pb.go")
endforeach()

set(qst_go_out_dir "${CMAKE_CURRENT_SOURCE_DIR}/go")

add_custom_command(
  OUTPUT ${qst_grpc_go_srcs}
  COMMAND ${CMAKE_COMMAND} -E make_directory "${qst_go_out_dir}"
  COMMAND ${_PROTOBUF_PROTOC}
  ARGS --go_out ${qst_go_out_dir}
  --go-grpc_out ${qst_go_out_dir}
  -I ${qst_proto_path}/home/kmdr/pro/qst/qst-front-end/qst-proto/src/daemon.proto /home/kmdr/pro/qst/qst-front-end/qst-proto/src/defs.proto /home/kmdr/pro/qst/qst-front-end/qst-proto/src/ext.proto/home/kmdr/pro/qst/qst-front-end/qst-proto/src/daemon.proto /home/kmdr/pro/qst/qst-front-end/qst-proto/src/defs.proto /home/kmdr/pro/qst/qst-front-end/qst-proto/src/ext.proto
  ${qst_proto_files}
  DEPENDS ${qst_proto_files}
  VERBATIM
)

add_custom_target(qst-grpc-cpp-gen ALL DEPENDS ${qst_grpc_cpp_srcs} ${qst_grpc_cpp_hdrs})
add_custom_target(qst-grpc-go-gen ALL DEPENDS ${qst_grpc_go_srcs})
add_library(qst-grpc-cpp STATIC)

set_target_properties(qst-grpc-cpp PROPERTIES
  ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/cpp"
  LIBRARY_OUTPUT_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/cpp"
)
target_sources(qst-grpc-cpp
  PRIVATE
  ${qst_grpc_cpp_srcs}
  PUBLIC
  ${qst_grpc_cpp_hdrs}
)

find_package(Protobuf CONFIG REQUIRED)
find_package(gRPC CONFIG REQUIRED)

target_link_libraries(qst-grpc-cpp PUBLIC
  gRPC::grpc++_reflection
  gRPC::grpc++
  protobuf::libprotobuf
)
