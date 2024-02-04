FROM rust:latest

RUN apt-get update && \
    apt-get install -y build-essential wget libssl-dev pkg-config lsb-release software-properties-common gnupg

RUN wget https://imagemagick.org/archive/ImageMagick.tar.gz && \
    tar -xvf ImageMagick.tar.gz && \
    cd ImageMagick-* && \
    ./configure && \
    make && \
    make install && \
    ldconfig /usr/local/lib

RUN apt-get install -y clang

ENV PKG_CONFIG_PATH=/usr/local/lib/pkgconfig
ENV LIBCLANG_PATH=/usr/lib/llvm-14/lib

WORKDIR /zaemon_bot

COPY Cargo.toml ./
COPY . .

ARG ENV_PREFIX
ARG ZAEMON_MONGODB_URI
ARG ZAEMON_DB_NAME
ARG ZAEMON_BOT_TOKEN

ENV ENV_PREFIX=$ENV_PREFIX
ENV ZAEMON_MONGODB_URI=$ZAEMON_MONGODB_URI
ENV ZAEMON_DB_NAME=$ZAEMON_DB_NAME
ENV ZAEMON_BOT_TOKEN=$ZAEMON_BOT_TOKEN

RUN cargo build --release

CMD ["./target/release/zaemon_bot"]
