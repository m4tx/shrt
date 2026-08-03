FROM rust:1.97 as builder
WORKDIR /usr/src/shrt
COPY . .
WORKDIR /usr/src/shrt/shrt-backend
RUN cargo install --path . --locked

FROM debian:13-slim
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/shrt-backend /usr/local/bin/shrt-backend
COPY config/prod.toml /app/config/prod.toml
CMD ["shrt-backend", "--listen", "0.0.0.0:8000", "--config", "prod"]
