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

RUN apk add --update --no-cache --repository http://dl-cdn.alpinelinux.org/alpine/v3.16/community \
    gn

RUN apk add --update --no-cache --repository http://dl-cdn.alpinelinux.org/alpine/edge/community \
    ninja-build

#
# RUN apk add --update --no-cache --repository http://dl-cdn.alpinelinux.org/alpine/edge/main \
#     llvm18 clang18
#
# # RUN export ASM="/usr/include/c++/10.3.1/aarch64-alpine-linux-musl/asm" && \
# #     mkdir -p ${ASM} && \
# #     touch ${ASM}/hwcap.h
#
#


# ------------- workflow dependencies -------------

# install rust
# REPLACE WITH: dtolnay/rust-toolchain@stable
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
ENV PATH="/root/.cargo/bin:${PATH}"


WORKDIR /code

# RUN git clone https://github.com/rust-skia/rust-skia.git

# COPY alpine-build.patch .
