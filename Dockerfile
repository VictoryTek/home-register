# Use official Rust image as builder
FROM rust:1.88 as builder
WORKDIR /app
COPY . .
# Build statically linked binary with musl
RUN apt-get update && apt-get install -y musl-tools \
    && rustup target add x86_64-unknown-linux-musl \
    && cargo build --release --target x86_64-unknown-linux-musl

# Use a minimal base image
FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/home-register .
COPY .env .env
CMD ["./home-register"]
