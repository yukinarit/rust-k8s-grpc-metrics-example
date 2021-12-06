FROM rust:latest as builder

WORKDIR /build
COPY . /build
RUN rustup component add rustfmt
RUN cd client && cargo build --release

FROM debian:buster-slim

RUN mkdir /app \
  && groupadd rust \
  && useradd -g rust rust \
  && mkdir -p rust

COPY --chown=rust:rust --from=builder /build/target/release/client /app
USER rust
ENV CLIENT_HOST=localhost \
    CLIENT_PORT=20000
CMD ["/app/client"]
