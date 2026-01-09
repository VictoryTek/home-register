# Stage 1: Build React frontend
FROM node:20-alpine as frontend-builder
WORKDIR /app/frontend
COPY frontend/package*.json ./
RUN npm ci
COPY frontend/ ./
RUN npm run build

# Stage 2: Build Rust backend
FROM rust:1.88 as backend-builder
WORKDIR /app
COPY . .
# Build statically linked binary with musl
RUN apt-get update && apt-get install -y musl-tools \
    && rustup target add x86_64-unknown-linux-musl \
    && cargo build --release --target x86_64-unknown-linux-musl

# Stage 3: Final image
FROM debian:buster-slim
WORKDIR /app
# Copy the binary
COPY --from=backend-builder /app/target/x86_64-unknown-linux-musl/release/home-registry .
# Copy built frontend to static directory
COPY --from=frontend-builder /app/frontend/dist ./static
# Copy migrations
COPY migrations ./migrations
COPY .env .env
EXPOSE 8210
CMD ["./home-registry"]
