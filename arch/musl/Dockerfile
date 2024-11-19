FROM node:16-alpine3.14

ENV RUSTFLAGS="-C target-feature=-crt-static" \
    CC="clang" \
    CXX="clang++" \
    GN_EXE="gn" \
    SKIA_GN_COMMAND="/usr/bin/gn" \
    SKIA_NINJA_COMMAND="/usr/bin/ninja"

RUN apk update && apk add --update --no-cache \
    bash curl git python3 perl build-base g++ llvm clang \
    musl-dev openssl-dev fontconfig-dev fontconfig ttf-dejavu


# ------------- workflow dependencies -------------

# replace with seanmiddleditch/gha-setup-ninja
RUN apk add --update --no-cache --repository http://dl-cdn.alpinelinux.org/alpine/v3.16/community gn
RUN apk add --update --no-cache --repository http://dl-cdn.alpinelinux.org/alpine/edge/community ninja-build


# # install rust
# # REPLACE WITH: dtolnay/rust-toolchain@stable
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
# ENV PATH="/root/.cargo/bin:${PATH}"

RUN adduser -u 1001 -M github
USER 1001

WORKDIR /code

# RUN git clone https://github.com/rust-skia/rust-skia.git

# COPY alpine-build.patch .
