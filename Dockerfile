FROM rust:latest

RUN apt-get update && apt-get install -y \
	cmake libclang-dev

WORKDIR /usr/src/srt-rs

# build dependencies with dummy main file
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./libsrt-sys/ ./libsrt-sys/
RUN mkdir src/ && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY ./ ./
RUN cargo build --release
