add_library(qst-core STATIC)

target_sources(qst-core
    PRIVATE
    core.cpp
)

# if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
# set(CMAKE_PREFIX_PATH "~/tools/qt/6.2.4/gcc_64" ${CMAKE_PREFIX_PATH})
# elseif(CMAKE_SYSTEM_NAME STREQUAL "Windows")
# set(CMAKE_PREFIX_PATH ${CMAKE_PREFIX_PATH})
# endif()

# find_package(QT 6.2.4 NAMES Qt6 Qt5 REQUIRED COMPONENTS Core Widgets)
# find_package(Qt${QT_VERSION_MAJOR} REQUIRED COMPONENTS Core Widgets)
add_subdirectory(comm)
add_subdirectory(ui)


target_link_libraries(
    qst-core
    PUBLIC
    qst-front-end-comm
    qst-front-end-ui
    Qt${QT_VERSION_MAJOR}::Widgets
)

target_include_directories(
    qst-core
    PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}
)
