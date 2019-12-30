project(myproject)
cmake_minimum_required(VERSION 2.8)

include_directories(/usr/local/include)

add_library(cproc SHARED cproc.c)
