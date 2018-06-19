#!/usr/bin/bash
set -e

check_protoc_version () {
    version="libprotoc $1"
    PROTOC="$HOME/protobuf/bin/protoc"
    if [ -f $PROTOC ]; then
        this_version=`$PROTOC --version`
        return `[ "$version" = "$this_version" ]`
    else
        return 1
    fi
}

if check_protoc_version '3.5.1'; then
    echo protoc version 3.5.1 detected.
    exit
fi

wget https://github.com/google/protobuf/archive/v3.5.1.tar.gz
tar -xzvf v3.5.1.tar.gz
cd protobuf-3.5.1 && ./autogen.sh && ./configure --prefix=$HOME/protobuf && make && make install
