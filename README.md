# test-tls-client

Two binaries to make requests using different certificate stores:
- Native (OS/Distribution defined CAs)
- WebPKI (Mozilla defined CAs)

## How to use

1. Download the applicable pre-built binary for a tag: <https://github.com/NickLarsenNZ/test-tls-client/tags>.
2. Set execute permission: `chmod +x <DOWNLOADED_BINARY>`.
3. Install it to your `$PATH`, or prefix with `./` to run from where you are.

<details>
<summary>Alternatively</summary>

If you have the rust toolchain, you can clone this repository and compile it directly.
Just replace the binaries in the instructions below with:

```shell
cargo run -p client-webpki-roots --release -- "$URL"
cargo run -p client-native-roots --release -- "$URL"
```

</details>

```shell
URL="https://wikipedia.org"

# Use the public WebPKI CAs
#
# This is expected to fail if you make a request to a URL signed by an internal
# PKI CA, or use a security element which breaks end-to-end encryption.
client-webpki-roots "$URL"

# Use the built-in CA store for your OS or distribution.
#
# This should work if you have an internal PKI and install certificates there on
# top of the default certificates.
client-native-roots "$URL"
```
