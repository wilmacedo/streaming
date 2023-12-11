FROM rust:1.71 as builder

RUN USER=root cargo new --bin app
WORKDIR /app

COPY ./Cargo.toml ./Cargo.lock ./

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/app*
RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /app/target/release/app /usr/local/bin/app

# COPY ../database.json /app/database.json

ENV DATABASE_PATH=/app/database.json

CMD ["./app"]