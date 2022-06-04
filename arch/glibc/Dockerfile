FROM node:buster-slim

RUN apt-get update && \
    apt-get install -y -q \
    python2 python3 perl git clang lldb lld \
    build-essential software-properties-common \
    libssl-dev libfontconfig-dev \
    ninja-build

RUN add-apt-repository "deb http://deb.debian.org/debian buster-backports main" && \
    apt-get update && apt-get install -y -t buster-backports \
    curl

ENV SKIA_NINJA_COMMAND="/usr/bin/ninja"

# RUN groupadd -r -g 1001 ghuser
# RUN useradd -r -u 1001 -g ghuser ghuser
# USER ghuser
