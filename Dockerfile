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

RUN cargo build --release


CMD ["./target/release/zaemon_bot"]
