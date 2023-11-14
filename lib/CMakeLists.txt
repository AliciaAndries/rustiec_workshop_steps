project(support)

cmake_minimum_required(VERSION 3.4.3)


add_library(tcpsock SHARED tcpsock.c)
target_compile_options(tcpsock PRIVATE ${COMMON_FLAGS})

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
   set(CARGO_CMD cargo build)
   set(OUT_LOCATION "debug")
else ()
   set(CARGO_CMD cargo build --release)
   set(OUT_LOCATION "release")
endif()

set(RUST_CODE_SO "${CMAKE_CURRENT_BINARY_DIR}/${OUT_LOCATION}/libvector.so")

add_custom_target(vector ALL
    COMMENT "Compiling vector"
    COMMAND CARGO_TARGET_DIR=${CMAKE_CURRENT_BINARY_DIR} ${CARGO_CMD} 
    COMMAND cp ${RUST_CODE_SO} ${CMAKE_CURRENT_BINARY_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
set_target_properties(vector PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})