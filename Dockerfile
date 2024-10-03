FROM rust:1.81 AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

COPY ./src ./src

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install -y \
    imagemagick \
    ghostscript \
    graphicsmagick

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/to_png .

EXPOSE 4000

CMD ["./to_png"]
