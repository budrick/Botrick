VERSION 0.8
IMPORT github.com/earthly/lib/rust:3.0.1 AS rust

FROM rust:alpine3.20
WORKDIR /botrickbuild

# build creates the binary target/release/example-rust
build:
    # CARGO function adds caching to cargo runs.
    # See https://github.com/earthly/lib/tree/main/rust
    # RUN apt update && apt install -y libssl-dev libsqlite3-dev pkg-config
    RUN apk add --no-cache findutils openssl-dev openssl-libs-static libcrypto3 sqlite-static=~3 sqlite-dev=~3 pkgconf musl-dev
    DO rust+INIT --keep_fingerprints=true
    COPY --keep-ts --dir crates Cargo.lock Cargo.toml .
    DO rust+CARGO --args="build --release --bin botrick" --output="release/[^/\.]+"
    SAVE ARTIFACT target/release/botrick botrick
    # SAVE ARTIFACT --keep-ts target/release/botrick AS LOCAL output/botrick

# docker creates docker image earthly/examples:rust
docker:
    FROM alpine:3.20
    COPY +build/botrick botrick
    # EXPOSE 9091
    ENTRYPOINT ["./botrick"]
    SAVE IMAGE --push botrick:latest
