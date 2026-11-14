FROM docker.io/library/rust:1-bookworm AS builder

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --locked

FROM docker.io/library/debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /build/target/release/oracle /usr/local/bin/oracle

ENV DATA_DIR=/app/data \
    RUST_LOG=info

USER 65532:65532

ENTRYPOINT ["/usr/local/bin/oracle"]
