cmake_minimum_required(VERSION 3.22)
project(rio_example C)

set(CMAKE_C_STANDARD 11)

set(RUST_LIB_PATH "${CMAKE_CURRENT_SOURCE_DIR}/../rio/target/x86_64-unknown-linux-gnu/release/librio.a")
set(RUST_INCLUDE_PATH "../rio")

add_executable(rio_example main.c)

target_include_directories(rio_example PRIVATE ${RUST_INCLUDE_PATH})

target_link_libraries(rio_example ${RUST_LIB_PATH})
