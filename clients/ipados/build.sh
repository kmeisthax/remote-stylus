#!/bin/zsh

# Build steps for iOS, because I hate Xcode.

cargo build --target aarch64-apple-ios

#todo: lipo, I would have done it here but I can't actually install the
#old toolchains with rustup

BUILDKIND=$1

if [ "$BUILDKIND" == "fastlane" ]
then
    rm -r ../../target/RemoteStylus-ipados
    rm ../../target/RemoteStylus.ipa
    mkdir ../../target/RemoteStylus-ipados
    cp -R bundle ../../target/RemoteStylus-ipados/RemoteStylus.app
    cp ../../target/aarch64-apple-ios/debug/remotestylus-ipados ../../target/RemoteStylus-ipados/RemoteStylus.app/remotestylus-ipados

    if [ -n "${IDENTITY+1}" ]
    then
        cp dev.mobileprovision.xml  ../../target/RemoteStylus-ipados/RemoteStylus.app/embedded.mobileprovision
        codesign -s $IDENTITY ../../target/RemoteStylus-ipados/Payload/RemoteStylus.app
    else
        echo "Team identity not specified, signing and packaging skipped."
        echo "You will not be able to install the application without one."
        echo "Please retry the command with IDENTITY specified."
    fi

    cd ../../target/RemoteStylus-ipados/
    zip -r ../RemoteStylus.ipa .
else
    echo "xcrun nonsense not yet implemented, please open Xcode and build the project"
fi