FROM docker.io/library/rust:1.69.0-bullseye

RUN rustup component add rustfmt clippy

RUN apt-get update \
    && apt-get install sqlite3 \
    && rm -rf /var/lib/apt/lists/*
