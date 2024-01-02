FROM rust:1.70 AS builder

WORKDIR /entsoe-rs
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update -y \
    && apt-get install -y ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /entsoe-rs/target/release/entsoe-rs /usr/local/bin/entsoe-rs

ENTRYPOINT ["/usr/local/bin/entsoe-rs"]
