FROM rust:latest AS builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /app/target/release /usr/local/bin

EXPOSE 3000

CMD ["food-service"]
