FROM rust:1.58-alpine as builder

WORKDIR /volume

RUN apk add --no-cache musl-dev=~1.2

COPY migrations/ migrations/
COPY src/ src/
COPY templates/ templates/
COPY Cargo.lock Cargo.toml ./

RUN cargo build --release && \
    strip --strip-all target/release/acralite

FROM alpine:3.15 as newuser

RUN echo "acralite:x:1000:" > /tmp/group && \
    echo "acralite:x:1000:1000::/dev/null:/sbin/nologin" > /tmp/passwd

FROM scratch

COPY --from=builder /volume/target/release/acralite /bin/
COPY --from=newuser /tmp/group /tmp/passwd /etc/

EXPOSE 8080
STOPSIGNAL SIGINT
USER acralite

ENTRYPOINT ["/bin/acralite"]
