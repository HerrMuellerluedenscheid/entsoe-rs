FROM rust:1.70

WORKDIR /entsoe-rs
COPY . .

RUN cargo install --path .

CMD ["entsoe-rs"]
