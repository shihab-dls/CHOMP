FROM docker.io/library/rust:1.73.0-bullseye AS build

WORKDIR /app

RUN apt-get update \
    && apt-get install -y \
        libopencv-dev clang libclang-dev

# Build dependencies
COPY Cargo.toml Cargo.lock ./
COPY chimp_chomp/Cargo.toml chimp_chomp/Cargo.toml
COPY chimp_protocol/Cargo.toml chimp_protocol/Cargo.toml
COPY clap_for_s3/Cargo.toml clap_for_s3/Cargo.toml
COPY graphql_endpoints/Cargo.toml graphql_endpoints/Cargo.toml
COPY graphql_event_broker/Cargo.toml graphql_event_broker/Cargo.toml
COPY graphql_types/Cargo.toml graphql_types/Cargo.toml
COPY opa_client/Cargo.toml opa_client/Cargo.toml
COPY pin_packing/Cargo.toml pin_packing/Cargo.toml
COPY soakdb_io/Cargo.toml soakdb_io/Cargo.toml
COPY soakdb_sync/Cargo.toml soakdb_sync/Cargo.toml
COPY targeting/Cargo.toml targeting/Cargo.toml
COPY the_paginator/Cargo.toml the_paginator/Cargo.toml
RUN mkdir chimp_chomp/src \
    && echo "fn main() {}" > chimp_chomp/src/main.rs \
    && mkdir chimp_protocol/src \
    && touch chimp_protocol/src/lib.rs \
    && mkdir clap_for_s3/src \
    && touch clap_for_s3/src/lib.rs \
    && mkdir graphql_endpoints/src \
    && touch graphql_endpoints/src/lib.rs \
    && mkdir graphql_event_broker/src \
    && touch graphql_event_broker/src/lib.rs \
    && mkdir graphql_types/src \
    && touch graphql_types/src/lib.rs \
    && mkdir opa_client/src \
    && touch opa_client/src/lib.rs \
    && mkdir pin_packing/src/ \
    && echo "fn main() {}" > pin_packing/src/main.rs \
    && mkdir soakdb_io/src \
    && touch soakdb_io/src/lib.rs \
    && mkdir soakdb_sync/src/ \
    && echo "fn main() {}" > soakdb_sync/src/main.rs \
    && mkdir targeting/src/ \
    && echo "fn main() {}" > targeting/src/main.rs \
    && mkdir the_paginator/src \
    && touch the_paginator/src/lib.rs \
    && cargo build --release

# Build workspace crates
COPY . /app
RUN touch chimp_chomp/src/lib.rs \
    && touch chimp_protocol/src/lib.rs \
    && touch clap_for_s3/src/lib.rs \
    && touch graphql_endpoints/src/lib.rs \
    && touch graphql_event_broker/src/lib.rs \
    && touch graphql_types/src/lib.rs \
    && touch opa_client/src/lib.rs \
    && touch pin_packing/src/main.rs \
    && touch soakdb_io/src/lib.rs \
    && touch soakdb_sync/src/main.rs \
    && touch targeting/src/main.rs \
    && touch the_paginator/src/lib.rs \
    && cargo build --release

# Collate dynamically linked shared objects for chimp_chomp
RUN mkdir /chimp_chomp_libraries \
    && cp \
        $(ldd /app/target/release/chimp_chomp | grep -o '/.*\.so\S*') \
        /app/target/release/libonnxruntime.so.* \
        /chimp_chomp_libraries

FROM gcr.io/distroless/cc as chimp_chomp

COPY chimp_chomp/chimp.onnx /chimp.onnx
COPY --from=build /chimp_chomp_libraries/* /lib
COPY --from=build /app/target/release/chimp_chomp /chimp_chomp

ENTRYPOINT ["/chimp_chomp"]

FROM gcr.io/distroless/cc as pin_packing

COPY --from=build /app/target/release/pin_packing /pin_packing

ENTRYPOINT ["/pin_packing"]

FROM gcr.io/distroless/cc as soakdb_sync

COPY --from=build /app/target/release/soakdb_sync /soakdb_sync

ENTRYPOINT ["/soakdb_sync"]

FROM gcr.io/distroless/cc as targeting

COPY --from=build /app/target/release/targeting /targeting

ENTRYPOINT ["/targeting"]
