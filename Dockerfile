FROM rust:1.78

WORKDIR /app
COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential && \
    cargo build --release --bin nebula_vrf_api

CMD ["./target/release/nebula_vrf_api"]
