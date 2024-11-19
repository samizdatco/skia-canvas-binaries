FROM node:16-alpine3.14

ENV RUSTFLAGS="-C target-feature=-crt-static"
# CC="clang" \
# CXX="clang++" \
# GN_EXE="gn" \
# SKIA_GN_COMMAND="/usr/bin/gn" \
# SKIA_NINJA_COMMAND="/usr/bin/ninja"

RUN apk update && apk add --update --no-cache \
    bash curl git python3 perl build-base g++ llvm clang \
    musl-dev openssl-dev fontconfig-dev fontconfig ttf-dejavu

RUN apk add --update --no-cache --repository http://dl-cdn.alpinelinux.org/alpine/v3.16/community gn
RUN apk add --update --no-cache --repository http://dl-cdn.alpinelinux.org/alpine/edge/community ninja-build

RUN adduser --uid 1001 --no-create-home --disabled-password --gecos "" github

# ------------- workflow dependencies -------------


# # install rust
# REPLACE WITH: dtolnay/rust-toolchain@stable
# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal && \
#     mkdir -p /root/.cargo/registry && \
#     chown -R github /root/.cargo/registry

# ENV PATH="/root/.cargo/bin:${PATH}"

# # install gh
# # MAYBE REPLACE WITH: https://github.com/dev-hanz-ops/install-gh-cli-action
# ARG GH_VERSION=2.55.0
# ARG GH_URL=https://github.com/cli/cli/releases/download/v$GH_VERSION/gh_${GH_VERSION}_linux_amd64.tar.gz
# RUN curl -sL $GH_URL | tar xz --strip-components=2 -C /usr/local/bin

# USER github

# WORKDIR /code
