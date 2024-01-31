#!/bin/bash
set -e
if [ "$1" = "" ]; then
    echo "Usage: $0 [cpp|go|clean]"
    exit 1
elif [ "$1" = "clean" ]; then
    rm -rf cpp
    exit 0
elif [ "$1" = "cpp" ]; then
    cmake --no-warn-unused-cli -DCMAKE_BUILD_TYPE:STRING=Release -DCMAKE_C_COMPILER=gcc -DCMAKE_CXX_COMPILER=g++ -S$PWD -Bbuild -G Ninja
    cmake --build build --config Release --target all --
    rm -rf build
elif [ "$1" = "go" ]; then
    protoc --go_out $PWD/../.. --go-grpc_out $PWD/../.. -I $PWD/src daemon.proto
    protoc --go_out $PWD/../.. --go-grpc_out $PWD/../.. -I $PWD/src defs.proto
    # protoc --go_out $PWD/../.. --go-grpc_out $PWD/../.. -I $PWD/src ext.proto
fi