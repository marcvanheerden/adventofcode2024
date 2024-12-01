cmake_minimum_required(VERSION 3.13)
include(pico_sdk_import.cmake)

project(blink_project)

pico_sdk_init()

add_executable(blink
    blink.c
)

# Link libraries
target_link_libraries(blink pico_stdlib)

# Enable USB output, UART output, etc.
pico_enable_stdio_usb(blink 1)
pico_enable_stdio_uart(blink 0)

# Create UF2 file 
pico_add_extra_outputs(blink)

