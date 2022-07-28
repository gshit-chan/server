FROM rust:1.62.1-slim-bullseye
WORKDIR /opt/chan_server
COPY . .
RUN cargo build --release
ENTRYPOINT target/release/server
