FROM rust:1.80 AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main(){}" > src/main.rs && cargo build --release
RUN rm -rf src
COPY . .
ENV CARGO_TARGET_DIR=/app/target
RUN cargo build --release --locked --bin burger-be

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/burger-be /app/burger-be
EXPOSE 3000
CMD ["/app/burger-be"]
