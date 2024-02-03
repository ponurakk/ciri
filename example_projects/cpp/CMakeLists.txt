cmake_minimum_required(VERSION 3.21)
project(example VERSION 0.1.0 LANGUAGES CXX)

set(SOURCE_FILES
  src/main.cpp
)

add_executable(${PROJECT_NAME} ${SOURCE_FILES})

target_include_directories(${PROJECT_NAME} PRIVATE include)