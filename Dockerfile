FROM rust:latest

RUN apt-get update && apt-get install -y \
	cmake libclang-dev

WORKDIR /usr/src/srt-rs
COPY ./ ./
RUN cargo build --release
