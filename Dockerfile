FROM rust:1-alpine3.21 AS builder

RUN apk add --no-cache musl-dev gcc ca-certificates

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

RUN mkdir /data

FROM scratch

COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder --chown=65532:65532 /data /data
COPY --from=builder /app/target/release/Rustick /rustick

USER 65532:65532
WORKDIR /data

CMD ["/rustick"]