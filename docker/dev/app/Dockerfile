FROM rust:1.98.0-trixie@sha256:7f7a53a25a0319dd8284e279d529d45759cb384d59b14cc6806132910f45522e

RUN useradd -m -u 1000 -s /bin/bash app_user \
 && rustup target add wasm32-unknown-unknown

USER app_user

# renovate: version=v0.7.10
RUN cargo install --locked \
    --git https://github.com/DioxusLabs/dioxus \
    --rev 57d6794ad60b949e5bd8aa282f6f8c3dc97a365e \
   dioxus-cli

COPY --chown=app_user . /srv/app
WORKDIR /srv/app

HEALTHCHECK CMD curl --fail -H "Accept: text/html" http://localhost:8000 || exit 1

CMD ["sleep", "infinity"]
