# GitFiend Server

This is the internal server process GitFiend uses to query repo data and run git commands.

#### Run server
`cargo run`

#### Generate typescript types
`cargo test`

#### Release build
`cargo build --release`

## Building static for linux

Default release build on linux doesn't work on CentOS 7.9 due to missing glibc (https://github.com/GitFiend/Support/issues/132).

Steps taken from https://blog.davidvassallo.me/2021/06/10/lessons-learned-building-statically-linked-rust-binaries-openssl/

```shell
### Compile static binary using rust

# 1. Update rustup
rustup update

# 2. Add some MUSL dependencies
sudo apt-get install pkg-config musl-tools

# 3. Add the Linux MUSL toolchain
rustup target add x86_64-unknown-linux-musl
```

`cargo build --target x86_64-unknown-linux-musl --release`
