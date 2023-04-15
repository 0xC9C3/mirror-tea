FROM rust:1.68.2-alpine3.17 as builder

WORKDIR /mirror-tea

COPY . .

RUN apk add --no-cache musl-dev openssl-dev

ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN cargo build --release

FROM alpine:3.17.3

COPY --from=builder /mirror-tea/target/release/mirror-tea /usr/local/bin/mirror-tea

RUN apk add --no-cache libgcc

ENTRYPOINT ["/usr/local/bin/mirror-tea"]