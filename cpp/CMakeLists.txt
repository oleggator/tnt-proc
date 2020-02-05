project(myproject CXX)
cmake_minimum_required(VERSION 3.12)

set(CMAKE_POSITION_INDEPENDENT_CODE ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_library(cppproc SHARED
        cppproc.cpp cppproc.h box/Context.cpp box/Context.h)
