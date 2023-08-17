cmake_minimum_required(VERSION 3.14)

project(qst-gui LANGUAGES CXX)


set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# set(CMAKE_PREFIX_PATH "/home/kmdr/tools/qt/6.2.4/gcc_64" ${CMAKE_PREFIX_PATH})

add_executable(qst-ui
  main.cpp
  select.cpp
  dialog.cpp
  edit.cpp
)

set_target_properties(qst-ui PROPERTIES
  AUTOMOC ON
  AUTORCC ON
  AUTOUIC ON
)

add_subdirectory(qst-proto)

target_include_directories(qst-ui PRIVATE
  ${CMAKE_CURRENT_SOURCE_DIR}
  ${CMAKE_CURRENT_SOURCE_DIR}/qst-proto
)

find_package(QT NAMES Qt6 Qt5 REQUIRED COMPONENTS Widgets)
find_package(Qt${QT_VERSION_MAJOR} REQUIRED COMPONENTS Widgets)

target_link_libraries(qst-ui PRIVATE
  qst-grpc-cpp
  Qt${QT_VERSION_MAJOR}::Widgets
  ${_REFLECTION}
  ${_GRPC_GRPCPP}
  ${_PROTOBUF_LIBPROTOBUF}
)

# find_package(ICU 73.0 COMPONENTS uc i18n REQUIRED)
# message(STATUS "!!!${ICU_LINK_LIBRARIES}")
install(TARGETS qst-ui
  LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
  RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)