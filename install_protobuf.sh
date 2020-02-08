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

PROTOC_FILENAME=protoc-${PROTOC_VERSION}-linux-x86_64.zip
mkdir -p $HOME/protobuf
pushd $HOME/protobuf
wget https://github.com/google/protobuf/releases/download/v${PROTOBUF_VERSION}/${PROTOC_FILENAME}
unzip ${PROTOC_FILENAME}
bin/protoc --version
popd
