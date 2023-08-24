
add_library(qst-front-end-comm STATIC)

target_sources(qst-front-end-comm
  PRIVATE
  comm.cpp
)

set_target_properties(qst-front-end-comm PROPERTIES
  AUTOMOC ON
  # AUTORCC ON
  # AUTOUIC ON
)

target_link_libraries(qst-front-end-comm
  PRIVATE
  Qt${QT_VERSION_MAJOR}::Core
  qst-grpc-cpp
)

target_include_directories(qst-front-end-comm PUBLIC
  ${PROJECT_SOURCE_DIR}/qst-proto/cpp
  ${CMAKE_CURRENT_SOURCE_DIR}
)

# qt_add_library(businesslogic STATIC
# businesslogic.cpp
# )
# target_link_libraries(businesslogic PRIVATE Qt6::Core)
# target_include_directories(businesslogic INTERFACE ${CMAKE_CURRENT_SOURCE_DIR})