cmake_minimum_required(VERSION 3.14)

project(qst-front-end LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(qst-f
  src/main.cpp
  src/select.cpp
  src/dialog.cpp
  src/edit.cpp
)

set_target_properties(qst-f PROPERTIES
  AUTOMOC ON
  AUTORCC ON
  AUTOUIC ON
)

add_subdirectory(qst-proto)

target_include_directories(qst-f PRIVATE
  ${CMAKE_CURRENT_SOURCE_DIR}
  ${CMAKE_CURRENT_SOURCE_DIR}/qst-proto
)

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
  set(CMAKE_PREFIX_PATH "~/tools/qt/6.2.4/gcc_64" ${CMAKE_PREFIX_PATH})
elseif(CMAKE_SYSTEM_NAME STREQUAL "Windows")
  set(CMAKE_PREFIX_PATH ${CMAKE_PREFIX_PATH})
endif()

find_package(QT 6.2.4 NAMES Qt6 Qt5 REQUIRED COMPONENTS Widgets)
find_package(Qt${QT_VERSION_MAJOR} REQUIRED COMPONENTS Widgets)

target_link_libraries(qst-f
  qst-grpc-cpp
  Qt${QT_VERSION_MAJOR}::Widgets
)

# find_package(ICU 73.0 COMPONENTS uc i18n REQUIRED)
# message(STATUS "!!!${ICU_LINK_LIBRARIES}")
install(TARGETS qst-f
  LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
  RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)