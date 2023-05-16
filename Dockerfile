FROM docker.io/library/rust:1.69.0-bullseye AS build

WORKDIR /app
COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc

COPY --from=build /app/target/release/backend /

ENTRYPOINT ["./backend"]
