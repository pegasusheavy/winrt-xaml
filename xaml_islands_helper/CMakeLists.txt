cmake_minimum_required(VERSION 3.20)
project(XamlIslandsHelper CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Add Windows SDK
set(CMAKE_SYSTEM_VERSION 10.0)

# Create the DLL
add_library(xaml_islands_helper SHARED
    src/xaml_islands_bridge.cpp
    src/xaml_islands_bridge.h
)

# Link Windows libraries
target_link_libraries(xaml_islands_helper
    WindowsApp
)

# Set output directory
set_target_properties(xaml_islands_helper PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/bin"
    LIBRARY_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib"
)

# Export symbols
target_compile_definitions(xaml_islands_helper PRIVATE XAML_ISLANDS_EXPORTS)

# Copy DLL to Rust target directory after build
add_custom_command(TARGET xaml_islands_helper POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E copy
    $<TARGET_FILE:xaml_islands_helper>
    "${CMAKE_SOURCE_DIR}/../target/debug/"
    COMMENT "Copying DLL to Rust target directory"
)


