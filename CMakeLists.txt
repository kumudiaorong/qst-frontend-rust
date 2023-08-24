cmake_minimum_required(VERSION 3.14)

project(qst-front-end LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

execute_process(
  COMMAND ${CMAKE_COMMAND} --no-warn-unused-cli -DCMAKE_BUILD_TYPE:STRING=Release -Bbuild -G Ninja
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/qst-proto
)
execute_process(
  COMMAND ${CMAKE_COMMAND} --build build --config Release --target qst-grpc-cpp-gen
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/qst-proto
)

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
  set(CMAKE_PREFIX_PATH "~/tools/qt/6.2.4/gcc_64" ${CMAKE_PREFIX_PATH})
elseif(CMAKE_SYSTEM_NAME STREQUAL "Windows")
  set(CMAKE_PREFIX_PATH ${CMAKE_PREFIX_PATH})
endif()

find_package(QT 6.2.4 NAMES Qt6 Qt5 REQUIRED COMPONENTS Widgets)
find_package(Qt${QT_VERSION_MAJOR} REQUIRED COMPONENTS Widgets)

add_executable(qst-f
  main.cpp
)

add_subdirectory(src)
add_subdirectory(qst-proto)

target_link_libraries(
  qst-f
  PUBLIC
  qst-core
)

# find_package(ICU 73.0 COMPONENTS uc i18n REQUIRED)
# message(STATUS "!!!${ICU_LINK_LIBRARIES}")
install(TARGETS qst-f
  LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
  RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)