# Remote Stylus for iPadOS

## Building

Remote Stylus supports two build paths: Xcode automatic signing and Fastlane.
Both require you to first build the project with `build.sh`.

Please note that if you have a free developer account, you will not be able to
use the Fastlane signing path. The magic that Xcode (and, for that matter,
AltStore) does to sign binaries without a valid team is not available in
Fastlane.

### Xcode

The Xcode project has automatic signing and provisioning enabled; which will
allow the installation of the app on a device. Note that it does not invoke
cargo for you at this time, you need to build the binary first with the script
and then build the project with Xcode.

You will also need to fill out an `identity.xcconfig` file with the variables
listed in `identity.sample`. If you are on a free dev account your bundle ID
needs to have your team ID in it too.

### Fastlane

If you specify `fastlane` as the first argument to `build.sh`, then we will
build a `RemoteStylus.ipa` that is properly signed with the code signing
identity listed in the `IDENTITY` variable.

NOTE: This is still a work in progress as I do not have a paid dev account at
this time.

## Codesigning identity tips

To get the SHA hash of a valid code signing certificate that you have already
obtained, run the following command:

```
security find-identity -pcodesigning -v
```

You will see one or more lines with a long string of hex digits, followed by
the name of the certificate. The hex digits are your SHA-1 hash and should be
copypasted directly into the `IDENTITY` variable as such:

```
IDENTITY=123ABC ./build.sh
```

If the identity was added correctly, you should see the script attempt to sign
the binaries. If you don't have a certificate you will need to get one signed
by Apple.

The final build product is an IPA, we assume you know how to get this onto a
device.