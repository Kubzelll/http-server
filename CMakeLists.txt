cmake_minimum_required(VERSION 3.27)
project(http_server)

set(CMAKE_CXX_STANDARD 20)

add_executable(http_server main.cpp
        http_tcp_server.cpp
        http_tcp_server.h)
