FROM alpine:3.14

ENV RUSTFLAGS="-C target-feature=-crt-static" \
    CC="clang" \
    CXX="clang++" \
    GN_EXE="gn" \
    SKIA_GN_COMMAND="/usr/bin/gn" \
    SKIA_NINJA_COMMAND="/usr/bin/ninja"

RUN apk update && apk add --update --no-cache \
    bash curl git python3 perl build-base g++ llvm clang \
    musl-dev openssl-dev fontconfig-dev fontconfig ttf-dejavu
