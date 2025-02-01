FROM rust:1.84 as builder
WORKDIR /usr/src/shrt
COPY . .
WORKDIR /usr/src/shrt/shrt-backend
RUN cargo install --path . --locked

FROM debian:12-slim
COPY --from=builder /usr/local/cargo/bin/shrt-backend /usr/local/bin/shrt-backend
ENV ROCKET_ADDRESS=0.0.0.0
CMD ["shrt-backend"]
