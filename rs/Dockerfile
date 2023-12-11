FROM rust:1.69-buster as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/rs .

COPY database.json /usr/local/bin/database.json

CMD ["./rs"]