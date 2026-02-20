# Stage 1: build frontend (Trunk -> WASM, output dist/)
FROM rust:1-bookworm AS frontend
RUN cargo install trunk
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY flamebolt ./flamebolt
COPY shared ./shared
COPY features ./features
WORKDIR /build/flamebolt
RUN trunk build

# Stage 2: build backend (Axum binary)
FROM rust:1-bookworm AS backend
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY flamebolt ./flamebolt
COPY shared ./shared
COPY features ./features
COPY --from=frontend /build/flamebolt/dist ./flamebolt/dist
ENV SKIP_TRUNK=1
RUN cargo build --release -p flamebolt

# Stage 3: runtime distroless
FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=backend /build/target/release/flamebolt ./flamebolt
COPY --from=frontend /build/flamebolt/dist ./dist
EXPOSE 6570
CMD ["./flamebolt"]
