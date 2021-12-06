FROM rust:slim-buster as builder

WORKDIR /build
COPY . /build
RUN rustup component add rustfmt
RUN cd server && cargo build --release

FROM debian:buster-slim

RUN mkdir /app \
  && groupadd rust \
  && useradd -g rust rust \
  && mkdir -p rust

COPY --chown=rust:rust --from=builder /build/target/release/server /app
USER rust
ENV SERVER_HOST=0.0.0.0 \
    SERVER_PORT=20000
CMD ["/app/server"]
