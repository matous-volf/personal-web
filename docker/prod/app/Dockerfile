FROM rust:1.97.1-alpine3.24@sha256:3c38f3f82c2f3d73da3b38e18d279393a04cb43ddded0e35088a8c3324d40900 AS builder

RUN apk add --no-cache \
    # renovate: repology=alpine_3_24/gcompat
    gcompat=1.1.0-r4 \
    # renovate: repology=alpine_3_24/musl-dev
    musl-dev=1.2.6-r2 \
    # renovate: repology=alpine_3_24/openssl-dev
    openssl-dev=3.5.7-r0 \
    # renovate: repology=alpine_3_24/openssl-libs-static
    openssl-libs-static=3.5.7-r0 \
    # renovate: version=v0.7.10
 && cargo install --locked \
     --git https://github.com/DioxusLabs/dioxus \
     --rev 57d6794ad60b949e5bd8aa282f6f8c3dc97a365e \
    dioxus-cli --features disable-telemetry

COPY . /srv/app
WORKDIR /srv/app

RUN dx bundle --release


FROM alpine:3.24.1@sha256:28bd5fe8b56d1bd048e5babf5b10710ebe0bae67db86916198a6eec434943f8b

COPY --from=builder /srv/app/target/dx/personal_web/release/web /srv/app

RUN chown -R 1000:1000 /srv/app

WORKDIR /srv/app

HEALTHCHECK CMD curl --fail -H "Accept: text/html" http://localhost || exit 1

USER 1000:1000

ENV IP="0.0.0.0"
ENV PORT="80"
CMD ["./server"]
