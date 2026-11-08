FROM rust:1.99.1-alpine3.24@sha256:7e9e2d068a107a2adafd19930011178d14a78326053cd4c3a9f7ad3fd049557d AS builder

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


FROM alpine:3.24.4@sha256:f59abe656b692d901e184be3445e084786602108599d2a51d229c0f5a48c71bf

COPY --from=builder /srv/app/target/dx/personal_web/release/web /srv/app

RUN chown -R 1000:1000 /srv/app

WORKDIR /srv/app

HEALTHCHECK CMD ["curl", "--fail", "-H", "Accept: text/html", "http://localhost"]

USER 1000:1000

ENV IP="0.0.0.0"
ENV PORT="80"
CMD ["./server"]
