FROM arm64v8/debian:buster

RUN apt-get update && \
    apt-get install -y -q \
    curl build-essential lsb-release wget software-properties-common \
    python2 libssl-dev libfontconfig-dev git clang lldb lld

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y --profile minimal
ENV PATH="/root/.cargo/bin:${PATH}"

RUN apt-get install -y ninja-build
ENV SKIA_NINJA_COMMAND="/usr/bin/ninja"

RUN groupadd -r -g 1000 pi
RUN useradd -r -u 1000 -g pi pi
USER pi
