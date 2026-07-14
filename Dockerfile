FROM rust:1.96-bookworm as builder

WORKDIR /app

COPY . .

RUN cargo build --release


FROM debian:bookworm-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/rust-crud-api .

EXPOSE 8080

CMD ["./rust-crud-api"]