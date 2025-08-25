# Use xx cross-compilation helper
FROM --platform=$BUILDPLATFORM tonistiigi/xx:1.6.1 AS xx

# Copy xx into rust:alpine image so we can use it. Also install clang, lld, gcc and musl-dev
# to support cross-compilation of C-language transitive dependencies.
FROM --platform=$BUILDPLATFORM rust:1.89-alpine AS builder
RUN apk add clang lld

COPY --from=xx / /

ARG TARGETPLATFORM

RUN xx-apk add gcc musl-dev
ENV XX_LIBC=musl

# Build botrick.
WORKDIR /usr/src/botrick
COPY . .

# We want to statically link sqlite3. Note we're using the rusqlite `bundled`
# feature to build the bundled copy in libsqlite3-sys
ENV SQLITE3_STATIC=1

# `cargo install` because we're just building a single binary here.
# We get release profile for free.
RUN xx-cargo install --path crates/botrick && \
    xx-verify --static /usr/local/cargo/bin/botrick

# Copy binary into a fresh new Alpine image
FROM scratch
COPY --from=builder /usr/local/cargo/bin/botrick /usr/local/bin/botrick

# Configure our default directories.
# RUN mkdir /certs /data
# ENV botrick_CERT_DIR=/certs
# WORKDIR /data

# Ready to go!
CMD ["/usr/local/bin/botrick"]
