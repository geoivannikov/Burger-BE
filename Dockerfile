FROM rust:latest as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/burger-be /app/burger-be

EXPOSE 3000

RUN chmod +x /app/burger-be

CMD ["sh", "-c", "./burger-be"]
