cmake_minimum_required(VERSION 3.17)
project(sceneformat)

set(CMAKE_CXX_STANDARD 20)

add_library(sceneformat scene.pb.cc scene.pb.h sceneio.cpp sceneio.h)