#!/bin/bash

build() {
    cargo build
}

build
if [[ "$1" == "buildwithrun" ]]; then
    ./run.sh
fi