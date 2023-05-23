FROM rust:1.67 AS builder
COPY . .
ARG SQLX_OFFLINE=true
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder ./target/release/menu-service ./target/release/menu-service
RUN apt-get update && apt install -y openssl
CMD ["/target/release/menu-service"]