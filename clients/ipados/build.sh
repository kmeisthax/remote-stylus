#!/bin/zsh

# Build steps for iOS, because I hate Xcode.

cargo build --target aarch64-apple-ios

#todo: lipo, I would have done it here but I can't actually install the
#old toolchains with rustup

rm -r ../../target/RemoteStylus-ipados
rm ../../target/RemoteStylus.ipa
mkdir ../../target/RemoteStylus-ipados
mkdir ../../target/RemoteStylus-ipados/Payload
cp -R bundle ../../target/RemoteStylus-ipados/Payload/RemoteStylus.app
cp ../../target/aarch64-apple-ios/debug/remotestylus-ipados ../../target/RemoteStylus-ipados/Payload/RemoteStylus.app/remotestylus-ipados

if [ -n "${IDENTITY+1}" ]
then
    codesign -s $IDENTITY ../../target/RemoteStylus-ipados/Payload/RemoteStylus.app
else
    echo "Codesign identity not specified, signing skipped."
    echo "To specify a code signing identity, run this script with the IDENTITY environment variable."
    echo "For the time being, we'll create an IPA but it won't have a signature."
fi

cd ../../target/RemoteStylus-ipados/
zip -r ../RemoteStylus.ipa .