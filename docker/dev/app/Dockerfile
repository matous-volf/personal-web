FROM rust:1.99.0-trixie@sha256:6ff07edce8775d0f64be7aba9197229407301bddf2054d62c27b541a6238a181

RUN useradd -m -u 1000 -s /bin/bash app_user \
    && rustup target add wasm32-unknown-unknown

# hadolint ignore=DL3066
USER app_user

# renovate: version=v0.7.10
RUN cargo install --locked \
    --git https://github.com/DioxusLabs/dioxus \
    --rev 57d6794ad60b949e5bd8aa282f6f8c3dc97a365e \
    dioxus-cli

COPY --chown=app_user . /srv/app
WORKDIR /srv/app

HEALTHCHECK CMD ["curl", "--fail", "-H", "Accept: text/html", "http://localhost:8000"]

CMD ["sleep", "infinity"]
