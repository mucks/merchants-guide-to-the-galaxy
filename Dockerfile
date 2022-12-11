# syntax=docker/dockerfile:1.3
FROM rust:1.65 AS builder

ARG TARGETPLATFORM

WORKDIR /root

RUN --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} \
	cargo install cargo-strip

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} --mount=type=cache,target=/root/target,id=${TARGETPLATFORM} \
	cargo build --release && \
	cargo strip && \
	mv /root/target/release/merchants-guide-to-the-galaxy /root


FROM debian:buster-slim
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /root/merchants-guide-to-the-galaxy /
ENTRYPOINT ["./merchants-guide-to-the-galaxy"]