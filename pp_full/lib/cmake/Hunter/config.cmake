hunter_config(CURL VERSION ${HUNTER_CURL_VERSION} CMAKE_ARGS HTTP_ONLY=ON CMAKE_USE_OPENSSL=OFF CMAKE_USE_LIBSSH2=OFF CURL_CA_PATH=none)
hunter_config(libjson-rpc-cpp VERSION ${HUNTER_libjson-rpc-cpp_VERSION} CMAKE_ARGS TCP_SOCKET_SERVER=ON)
hunter_config(
    Boost
    VERSION 1.80.0_new_url
    SHA1 4ff012adaa7d8952913be05bf4233005de48289c
    URL https://boostorg.jfrog.io/artifactory/main/release/1.80.0/source/boost_1_80_0.tar.gz
)
