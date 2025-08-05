FROM node:bullseye-slim AS build-git
ARG TARGETARCH

# upgrade git to 2.50
RUN apt-get update && apt-get install -y -q \
  build-essential software-properties-common \
  libssl-dev libexpat1-dev libghc-zlib-dev libcurl4-gnutls-dev gettext

ARG GIT_VERSION=2.50.1
ARG GIT_URL=https://github.com/git/git/archive/refs/tags/v$GIT_VERSION.tar.gz
RUN curl -sL $GIT_URL | tar xz -C /opt && \
  cd /opt/git-$GIT_VERSION && \
  NO_TCLTK=1 make -j 12 prefix=/usr/local install && \
  rm -rf /opt/git-$GIT_VERSION

# install gh
ARG GH_VERSION=2.55.0
ARG GH_URL=https://github.com/cli/cli/releases/download/v$GH_VERSION/gh_${GH_VERSION}_linux_$TARGETARCH.tar.gz
RUN curl -sL $GH_URL | tar xz --strip-components=1 -C /usr/local

# ----

FROM node:bullseye-slim AS build-skia
COPY --from=build-git /usr/local /usr/local

# basics & skia deps
RUN apt-get update && \
    apt-get install -y -q \
        wget curl perl build-essential software-properties-common \
        libssl-dev libfontconfig-dev ninja-build

# install rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
ENV PATH="/root/.cargo/bin:$PATH"

WORKDIR /opt

# install clang v18
ARG LLVM_VERSION=18
COPY install-llvm.sh .
RUN bash install-llvm.sh $LLVM_VERSION
