set(CMAKE_AR ${CMAKE_CURRENT_LIST_DIR}/vendor/x86_64-linux-musl-native/bin/ar CACHE FILEPATH "x86_64-linux-musl archiver")
set(CMAKE_EXE_LINKER_FLAGS "-B${CMAKE_CURRENT_LIST_DIR}/vendor/x86_64-linux-musl-native/bin" CACHE FILEPATH "x86_64-linux-musl exe_linker")
set(CMAKE_MODULE_LINKER_FLAGS "-B${CMAKE_CURRENT_LIST_DIR}/vendor/x86_64-linux-musl-native/bin" CACHE FILEPATH "x86_64-linux-musl module_linker")
set(CMAKE_SHARED_LINKER_FLAGS "-B${CMAKE_CURRENT_LIST_DIR}/vendor/x86_64-linux-musl-native/bin" CACHE FILEPATH "x86_64-linux-musl shared_linker")
