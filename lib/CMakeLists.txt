project(support)

cmake_minimum_required(VERSION 3.4.3)


add_library(tcpsock SHARED tcpsock.c)
target_compile_options(tcpsock PRIVATE ${COMMON_FLAGS})

add_library(vector SHARED vector.c)
target_compile_options(vector PRIVATE ${COMMON_FLAGS})