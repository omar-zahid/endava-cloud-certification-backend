FROM rust:1.92-bookworm AS builder
WORKDIR /usr/src/axum-web
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/axum-web /usr/local/bin/axum-web
EXPOSE 3000
CMD ["axum-web"]
