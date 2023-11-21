FROM rust:1.70 AS builder

WORKDIR /entsoe-rs
COPY . .

RUN cargo build --release

FROM alpine:latest
COPY --from=builder /entsoe-rs/target/release/entsoe-rs /usr/local/bin/entsoe-rs

CMD ["entsoe-rs"]
