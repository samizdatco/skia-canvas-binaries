FROM node:buster-slim

RUN apt-get update && \
    apt-get install -y -q \
    python2 python3 perl git clang lldb lld \
    build-essential software-properties-common \
    libssl-dev libfontconfig-dev \
    ninja-build

RUN add-apt-repository "deb http://deb.debian.org/debian buster-backports main" && \
    apt-get update && apt-get install -t buster-backports -y -q \
    curl

WORKDIR /usr/local/src
RUN git clone https://gn.googlesource.com/gn && \
    cd gn && \
    python build/gen.py && \
    ninja -C out && \
    cp out/gn /usr/local/bin/gn && \
    rm -rf /usr/local/src/gn

ENV SKIA_GN_COMMAND="/usr/local/bin/gn"
ENV SKIA_NINJA_COMMAND="/usr/bin/ninja"

# USER node