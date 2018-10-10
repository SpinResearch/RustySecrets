#!/usr/bin/env bash
set -e
set -x

PROTOC_VERSION=$(cat PROTOC_VERSION)

check_protoc_version () {
    this_version=`protoc --version`
    return `[ "libprotoc $PROTOC_VERSION" = "$this_version" ]`
}

if check_protoc_version; then
    echo $PROTOC_VERSION detected.
    exit
fi

wget https://github.com/google/protobuf/archive/v$PROTOC_VERSION.tar.gz
tar -xzf v$PROTOC_VERSION.tar.gz
cd protobuf-$PROTOC_VERSION
./autogen.sh >/dev/null 2>&1
./configure --prefix=$HOME/protobuf >/dev/null 2>&1
make >/dev/null 2>&1
make install >/dev/null 2>&1

