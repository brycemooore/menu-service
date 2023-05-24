FROM rust:1.67 AS builder
WORKDIR /usr/src/menu-service
COPY . .
ARG SQLX_OFFLINE=true
RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10
COPY --from=builder /usr/local/cargo/bin/menu-service /usr/local/bin/menu-service
CMD ["menu-service"]