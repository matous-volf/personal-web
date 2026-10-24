FROM rust:1.99.1-alpine3.24@sha256:b1d5dd5a22a09404d3e798c0cf051b689077360374eb505fa5bb93c479693d87 AS builder

RUN apk add --no-cache \
    # renovate: repology=alpine_3_24/gcompat
    gcompat=1.1.0-r4 \
    # renovate: repology=alpine_3_24/musl-dev
    musl-dev=1.2.6-r3 \
    # renovate: repology=alpine_3_24/openssl-dev
    openssl-dev=3.5.9-r0 \
    # renovate: repology=alpine_3_24/openssl-libs-static
    openssl-libs-static=3.5.9-r0 \
    # renovate: version=v0.7.10
    && cargo install --locked \
    --git https://github.com/DioxusLabs/dioxus \
    --rev 57d6794ad60b949e5bd8aa282f6f8c3dc97a365e \
    dioxus-cli --features disable-telemetry

COPY . /srv/app
WORKDIR /srv/app

RUN dx bundle --release


FROM alpine:3.24.3@sha256:be2656061b536c1883c4034790e21d0a09c5aaaf51968b89368a3c0371ea2c9a

COPY --from=builder /srv/app/target/dx/personal_web/release/web /srv/app

RUN chown -R 1000:1000 /srv/app

WORKDIR /srv/app

HEALTHCHECK CMD ["curl", "--fail", "-H", "Accept: text/html", "http://localhost"]

USER 1000:1000

ENV IP="0.0.0.0"
ENV PORT="80"
CMD ["./server"]
