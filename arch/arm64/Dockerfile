FROM arm64v8/debian:buster-slim

RUN apt-get update && \
    apt-get install -y -q \
    curl build-essential lsb-release wget software-properties-common \
    python2 libssl-dev libfontconfig-dev git clang lldb lld ninja-build

ENV SKIA_NINJA_COMMAND="/usr/bin/ninja"

RUN groupadd -r -g 1000 pi
RUN useradd -r -u 1000 -g pi pi

WORKDIR /code
RUN chown pi:pi /code

USER pi
RUN export CARGO_HOME=/code/.cargo &&
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
ENV PATH="$CARGO_HOME/bin:${PATH}"

