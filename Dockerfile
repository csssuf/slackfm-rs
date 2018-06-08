FROM rustlang/rust:nightly as build
# Build a new project and just build dependencies in order to cache them.
RUN USER=root cargo new --bin slackfm
WORKDIR /slackfm
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
# Delete the placeholder source, add our own, and rebuild.
RUN rm src/*.rs target/release/slackfm
COPY ./src ./src
COPY ./migrations ./migrations
RUN cargo build --release

# Copy into a new slimmer image.
FROM debian:stretch-slim
RUN apt-get update && \
    apt-get install -yy \
        ca-certificates \
        libpq5 \
        libssl1.1
COPY --from=build slackfm/target/release/slackfm .
CMD ["./slackfm"]
