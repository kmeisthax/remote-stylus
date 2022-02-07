# Remote Stylus for iPadOS

## Building

Run the `build.sh` script. This will produce a `RemoteStylus.ipa` directory.

If you specify the SHA-1 or SHA-256 hash of your code signing certificate in
`IDENTITY`, the script will also sign the bundle for you. Otherwise, you're on
your own.

To get the SHA hash of a valid code signing certificate, run the following
command:

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