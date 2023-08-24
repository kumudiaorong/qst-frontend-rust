add_library(qst-front-end-ui STATIC)

target_sources(qst-front-end-ui
  PRIVATE
  select.cpp
  dialog.cpp
  edit.cpp
)

set_target_properties(qst-front-end-ui PROPERTIES
  AUTOMOC ON
  # AUTORCC ON
  # AUTOUIC ON
)

target_link_libraries(qst-front-end-ui
  PRIVATE
  Qt${QT_VERSION_MAJOR}::Widgets
)

target_include_directories(qst-front-end-ui PUBLIC
  ${PROJECT_SOURCE_DIR}/qst-proto/cpp
  ${CMAKE_CURRENT_SOURCE_DIR}
)
