FROM rust:bookworm AS builder

WORKDIR /app

COPY . .

RUN cargo build --release --package zanistarast-mira-api


FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY . .

COPY --from=builder \
    /app/target/release/zanistarast-mira-api \
    /usr/local/bin/zanistarast-mira-api

ENV PORT=10000
ENV MIRA_REPOSITORY_ROOT=/app

EXPOSE 10000

CMD ["zanistarast-mira-api"]


