FROM debian:stable-slim
MAINTAINER Darwinia hello@darwinia.network

RUN apt-get update && apt-get -y install curl cmake pkg-config libssl-dev git clang libclang-dev && apt-get clean
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup default nightly
RUN rustup update
RUN rustup target add wasm32-unknown-unknown
