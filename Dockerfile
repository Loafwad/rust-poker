FROM rust:buster as builder

ENV SQLX_OFFLINE true

RUN rustup update

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && touch src/lib.rs
RUN cargo build --release

RUN rm -rf src
COPY src ./src
RUN cargo build --release

FROM debian:buster

ENV APP poker

RUN apt update                      \
    && apt install -y libssl1.1     \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /target/release/$APP /usr/local/bin/$APP

EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/backend"]
