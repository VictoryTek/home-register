# =============================================================================
# Home Registry - Hardened Multi-Stage Dockerfile
# =============================================================================
# Security features:
# - Pinned base images with specific versions
# - Non-root user in final stage
# - Minimal final image (distroless-style)
# - No shell access in production
# - Health check included
# =============================================================================

# ------------------------------------------------------------------------------
# Stage 1: Build React Frontend
# ------------------------------------------------------------------------------
FROM node:20.18-alpine3.20 AS frontend-builder

# Set working directory
WORKDIR /app/frontend

# Copy package files first for better layer caching
COPY frontend/package*.json ./

# Install dependencies with locked versions
RUN npm ci --ignore-scripts && npm cache clean --force

# Copy source code
COPY frontend/ ./

# Build production bundle
RUN npm run build

# ------------------------------------------------------------------------------
# Stage 2: Build Rust Backend
# ------------------------------------------------------------------------------
FROM rust:1.88-bookworm AS backend-builder

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests first for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create dummy source for dependency compilation
RUN mkdir src && echo 'fn main() { println!("Dummy"); }' > src/main.rs

# Build dependencies only (cached layer)
RUN cargo build --release && rm -rf src target/release/deps/home_registry*

# Copy actual source code
COPY src ./src

# Build the actual application
RUN cargo build --release --locked

# Strip the binary for smaller size
RUN strip target/release/home-registry

# ------------------------------------------------------------------------------
# Stage 3: Final Production Image
# ------------------------------------------------------------------------------
FROM debian:bookworm-20241223-slim AS runtime

# Labels for container metadata
LABEL org.opencontainers.image.title="Home Registry"
LABEL org.opencontainers.image.description="Home inventory management system"
LABEL org.opencontainers.image.version="0.1.0"
LABEL org.opencontainers.image.licenses="MIT"

# Install runtime dependencies only
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create non-root user for security
RUN groupadd --gid 1000 appgroup \
    && useradd --uid 1000 --gid appgroup --shell /bin/false --create-home appuser

# Set working directory
WORKDIR /app

# Copy the compiled binary from builder
COPY --from=backend-builder --chown=appuser:appgroup /app/target/release/home-registry ./

# Copy built frontend to static directory
COPY --from=frontend-builder --chown=appuser:appgroup /app/frontend/dist ./static

# Copy migrations (read-only)
COPY --chown=appuser:appgroup migrations ./migrations

# Set proper permissions
RUN chmod 755 /app/home-registry \
    && chmod -R 644 /app/migrations/* \
    && chmod 755 /app/migrations

# Switch to non-root user
USER appuser

# Expose application port
EXPOSE 8210

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD ["/app/home-registry", "--health-check"] || exit 1

# Run the binary
# Note: Use exec form to ensure signals are properly handled
CMD ["./home-registry"]
