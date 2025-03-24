#!/usr/bin/env just --justfile

clean:
    ./gradlew clean library:clean convention-plugins:clean cleanNativeDistributionCommonization
    rm -rf build
    rm -rf library/build
    rm -rf convention-plugins/build
    rm -rf .gradle

test:
    ./gradlew :library:macosArm64Test
    leaks --atExit --list -- library/build/bin/macosArm64/debugTest/test.kexe

leaks:
    leaks --atExit --list -- library/build/bin/macosArm64/debugTest/test.kexe

re-test:
    cd rio && just all
    ./gradlew clean
    ./gradlew :library:macosArm64Test
    leaks --atExit --list -- library/build/bin/macosArm64/debugTest/test.kexe

re-interop:
    cd rio && just all
    ./gradlew :library:cleanInterop
    ./gradlew :library:copyCommonizeCInteropForIde
