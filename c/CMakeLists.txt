project(myproject C)
cmake_minimum_required(VERSION 3.12)

set(CMAKE_POSITION_INDEPENDENT_CODE ON)

add_library(cproc SHARED cproc.c)
target_link_libraries(cproc msgpuck)
