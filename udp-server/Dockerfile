FROM rust:1.81-bullseye AS builder
WORKDIR /usr/src/udp-server
COPY . .
RUN cargo install --path .

FROM debian:11.11-slim
RUN apt-get update && apt-get install -y gcc-multilib
COPY --from=builder /usr/local/cargo/bin/udp-server /usr/local/bin/udp-server
CMD ["udp-server"]

EXPOSE 34254/udp
