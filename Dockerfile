FROM ubuntu:24.04

RUN apt update && apt install -y g++-arm-linux-gnueabihf curl build-essential protobuf-compiler
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
RUN rustup target add armv7-unknown-linux-gnueabihf
