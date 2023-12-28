cmake_minimum_required(VERSION 3.5)

set(X_VCPKG_APPLOCAL_DEPS_INSTALL ON)
set(VCPKG_APPLOCAL_DEPS_INSTALL ON)

set(VCPKG_TARGET_TRIPLET x64-windows-static-md)
set(CMAKE_TOOLCHAIN_FILE "$ENV{VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake"
  CACHE STRING "Vcpkg toolchain file")

project(coiio)

set(CMAKE_CXX_STANDARD 14)

add_library(coiio STATIC coiio.cpp)
# target_include_directories(coiio PUBLIC ${INC_OIIO} ${INC_OPENEXR})
# target_link_libraries(coiio PUBLIC 
#     ${LIB_OIIO} 
#     ${LIB_ILMIMF} 
#     ${LIB_IMATH} 
#     ${LIB_IEX} 
#     ${LIB_HALF})
# target_compile_options(coiio PRIVATE "-Wno-deprecated-register" "-D_GLIBCXX_USE_CXX11_ABI=0")

find_package(OpenImageIO CONFIG REQUIRED)

target_link_libraries(coiio PUBLIC
    OpenImageIO::OpenImageIO)

# install(TARGETS coiio DESTINATION ${CMAKE_INSTALL_PREFIX})
install(TARGETS coiio DESTINATION .)

