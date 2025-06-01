FROM rust:1.87 as builder
RUN rustup target add wasm32-unknown-unknown && \
     cargo install trunk
WORKDIR /usr/src/shrt
COPY . .
WORKDIR /usr/src/shrt/shrt-frontend
ENV SHRT_API_URL=/api
RUN trunk build --release

FROM nginx:1.27
COPY --from=builder /usr/src/shrt/shrt-frontend/dist /usr/share/nginx/html
