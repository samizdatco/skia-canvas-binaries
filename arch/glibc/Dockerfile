FROM node:buster-slim

RUN apt-get update && \
    apt-get install -y -q \
    curl python2 python3 perl git clang lldb lld \
    build-essential software-properties-common \
    libssl-dev libfontconfig-dev \
    ninja-build

ENV SKIA_NINJA_COMMAND="/usr/bin/ninja"

RUN groupadd -r -g 1001 ghuser
RUN useradd -r -u 1001 -g ghuser ghuser
USER ghuser
