# Stage 1: build frontend (Trunk -> WASM in flamebolt/client, output dist/)
FROM rust:1-bookworm AS frontend
RUN cargo install trunk
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY flamebolt ./flamebolt
COPY shared ./shared
COPY features ./features
WORKDIR /build/flamebolt/client
RUN trunk build

# Stage 2: build backend (Axum binary flamebolt_server)
FROM rust:1-bookworm AS backend
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY flamebolt ./flamebolt
COPY shared ./shared
COPY features ./features
RUN cargo build --release -p flamebolt_server

# Stage 3: runtime (frontend + API no mesmo container)
FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=backend /build/target/release/flamebolt_server ./flamebolt_server
COPY --from=frontend /build/flamebolt/client/dist ./dist
EXPOSE 6570
CMD ["./flamebolt_server"]
