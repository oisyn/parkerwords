cmake_minimum_required(VERSION 3.14)

project(
    ParkerWords
    LANGUAGES C CXX
    VERSION 0.1.0)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(parker-words parkerwords.cpp)

target_compile_features(parker-words PUBLIC cxx_std_20)
target_compile_options(parker-words PUBLIC -mavx -mavx2)
