FROM rust:1.78

# Create working directory
WORKDIR /app
COPY . .

# install deps
RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential

# compile binary
RUN cargo build --release --bin nebula_vrf_api

# set correct startup binary path
CMD ["./target/release/nebula_vrf_api"]
