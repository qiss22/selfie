FROM rust:1.81.0-slim as chef
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.81.0-slim as cacher
WORKDIR /app
COPY recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.81.0-slim as builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/auth-service /usr/local/bin/
CMD ["auth-service"]
