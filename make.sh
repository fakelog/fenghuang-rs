#!/bin/bash

build() {
    cargo build
}

if [[ "$1" != "-r" ]]; then
    build
    if [[ "$1" == "-br" ]]; then
        ./run.sh
    fi
fi

if [[ "$1" == "-r" ]]; then
    ./run.sh
fi