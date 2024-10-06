#!/usr/bin/env just --justfile

clean:
    ./gradlew clean library:clean convention-plugins:clean cleanNativeDistributionCommonization
    rm -rf build
    rm -rf library/build
    rm -rf convention-plugins/build
    rm -rf .gradle
