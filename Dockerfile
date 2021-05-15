FROM rustlang/rust:nightly-stretch-slim as builder
# Current 1.5x does not start the application correctly
#FROM rust:1.52.1-slim-bullseye as builder

RUN apt-get update \
    && apt-get install -y musl-tools \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/webapp
COPY src src
COPY Cargo.* ./

# compile with musl and strip afterwards to reduce size
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl
RUN strip target/x86_64-unknown-linux-musl/release/pandoc-rustful-api

###############
# Web Container
###############
FROM pandoc/latex:latest

RUN addgroup -g 1000 webapp \
    && adduser -D -s /bin/sh -u 1000 -G webapp webapp

WORKDIR /home/webapp/app/
COPY --chown=webapp:webapp --from=builder /usr/src/webapp/target/x86_64-unknown-linux-musl/release/pandoc-rustful-api .
COPY Rocket.toml .

# Switch to user and start the webservice
USER webapp

EXPOSE 8000
ENTRYPOINT ["./pandoc-rustful-api"]