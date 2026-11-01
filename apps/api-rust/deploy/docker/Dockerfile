FROM rust:1.97-bookworm AS builder

RUN apt-get update \
    && apt-get install --yes --no-install-recommends libdbus-1-dev libssl-dev pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace
COPY apps/api-rust/ apps/api-rust/
RUN cargo build --manifest-path apps/api-rust/Cargo.toml --locked --release \
    -p orbis-server --bin orbis-server

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates curl libdbus-1-3 libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --home-dir /nonexistent \
        --no-create-home --shell /usr/sbin/nologin orbis \
    && install -d -o 10001 -g 10001 -m 0750 /var/lib/orbis/license

COPY --from=builder /workspace/apps/api-rust/target/release/orbis-server /usr/local/bin/orbis-server

ENV BIND_HOST=0.0.0.0 \
    PORT=3000 \
    ORBIS_SERVER_CONFIG_FILE=/etc/orbis/server.json \
    ORBIS_LICENSE_DATA_DIR=/var/lib/orbis/license

USER 10001:10001
EXPOSE 3000
VOLUME ["/var/lib/orbis/license"]

HEALTHCHECK --interval=30s --timeout=5s --start-period=60s --retries=3 \
    CMD curl --fail --silent --show-error http://127.0.0.1:3000/api/v1/health

ENTRYPOINT ["/usr/local/bin/orbis-server"]
