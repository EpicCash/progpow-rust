cmake_minimum_required(VERSION 3.19)

include_directories(/usr/local/cuda/include /opt/cuda/include)



# include(cmake/cable/bootstrap.cmake)

#include(CableBuildInfo)
#include(CableBuildType)
#include(CableToolchains)
#include(HunterGate)

#if(NOT APPLE)
#  set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -fpic -fPIC" CACHE INTERNAL "")
#  set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -fpic -fPIC" CACHE INTERNAL "")
#  set(CMAKE_POSITION_INDEPENDENT_CODE ON)
#endif()

#include(defaults/HunterCacheServers)

# cable_configure_toolchain(DEFAULT cxx11)

#set(HUNTER_CONFIGURATION_TYPES Release)
#HunterGate(
#    URL "https://github.com/cpp-pm/hunter/archive/v0.25.8.tar.gz"
#    SHA1 "26c79d587883ec910bce168e25f6ac4595f97033"
#)
# Set the toolchain file via -DCMAKE_TOOLCHAIN_FILE=... or environment variable.
# Example: cmake -DCMAKE_TOOLCHAIN_FILE=/path/to/vcpkg/scripts/buildsystems/vcpkg.cmake ..
# Uncomment and set the following line if you want to hardcode a default (not recommended):
# set(CMAKE_TOOLCHAIN_FILE "/absolute/path/to/vcpkg/scripts/buildsystems/vcpkg.cmake")
project(progpow)
set(PROJECT_VERSION 0.15.0.dev0)

#set(BOOST_ROOT "./3rdparty/boost_1_87_0/boost" )
#set(Boost_INCLUDE_DIR "3rdparty/boost_1_87_0/boost")
#set(Boost_NO_SYSTEM_PATHS on CACHE BOOL "Do not search system for Boost" )

#set(Boost_USE_STATIC_LIBS OFF) 
#set(Boost_USE_MULTITHREADED ON)  
#set(Boost_USE_STATIC_RUNTIME OFF) 


#set(Boost_NO_BOOST_CMAKE YES)





#cable_set_build_type(DEFAULT Release CONFIGURATION_TYPES Release RelWithDebInfo Debug)

# link_directories interprets relative paths with respect to CMAKE_CURRENT_SOURCE_DIR
#cmake_policy(SET CMP0015 NEW)

# let cmake autolink dependencies on windows
# it's specified globally, cause qt libraries requires that on windows and they are also found globally
#cmake_policy(SET CMP0020 NEW)

# 3.1 and above
cmake_policy(SET CMP0167 NEW)

######################################################################################################

option(ETHASHCL "Build with OpenCL mining" ON)
option(ETHASHCUDA "Build with CUDA mining" OFF)
option(ETHDBUS "Build with D-Bus support" OFF)
option(APICORE "Build with API Server support" ON)

# propagates CMake configuration options to the compiler
function(configureProject)
	if (ETHASHCL)
		add_definitions(-DETH_ETHASHCL)
	endif()
	if (ETHASHCUDA)
	 	add_definitions(-DETH_ETHASHCUDA)
	endif()
endfunction()

#hunter_add_package(Boost COMPONENTS system)
#
#find_package(boost_system CONFIG REQUIRED)
#hunter_add_package(jsoncpp)
#find_package(jsoncpp CONFIG REQUIRED)

#hunter_add_package(libjson-rpc-cpp)
#find_package(libjson-rpc-cpp CONFIG REQUIRED)

#hunter_add_package(CppNetlibUri)
#find_package(skyr-url CONFIG REQUIRED)

#if(Boost_FOUND)
#    include_directories(${Boost_INCLUDE_DIRS}) 
  #  add_executable(progname file1.cxx file2.cxx) 
   # target_link_libraries(progname ${Boost_LIBRARIES})
#endif()

configureProject()

message("------------------------------------------------------------------------")
message("-- CMake ${CMAKE_VERSION}")
message("-- Build ${CMAKE_BUILD_TYPE} / ${CMAKE_SYSTEM_NAME}")
message("------------------------------------------------------------- components")
message("-- ETHASHCL         Build OpenCL components                  ${ETHASHCL}")
message("-- ETHASHCUDA       Build CUDA components                    ${ETHASHCUDA}")
message("------------------------------------------------------------------------")
message("")


include(EthCompilerSettings.cmake)

add_subdirectory(libprogpow)
add_subdirectory(libdevcore)
add_subdirectory(libethash)
add_subdirectory(libethcore)
add_subdirectory(libexternal)

if (ETHASHCL)
	add_subdirectory(libethash-cl)
endif ()
if (ETHASHCUDA)
	add_subdirectory(libethash-cuda)
endif ()

if(WIN32)
	set(CPACK_GENERATOR ZIP)
else()
	set(CPACK_GENERATOR TGZ)
endif()
set(CPACK_PACKAGE_FILE_NAME ${PROJECT_NAME}-${PROJECT_VERSION}-${CMAKE_SYSTEM_NAME})
set(CPACK_PACKAGE_CHECKSUM SHA256)
set(CPACK_INCLUDE_TOPLEVEL_DIRECTORY FALSE)
include(CPack)


# Export some variables for CIs
if(PROJECT_VERSION MATCHES "^([0-9]+)\\.([0-9]+)\\.([0-9]+)$")
	set(PROJECT_VERSION_IS_PRERELEASE false)
else()
	set(PROJECT_VERSION_IS_PRERELEASE true)
endif()
#configure_file(cmake/variables.sh.in variables.sh)
#configure_file(cmake/variables.ps1.in variables.ps1)
