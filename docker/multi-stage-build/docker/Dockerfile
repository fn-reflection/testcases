FROM rust:1.78.0 AS base


FROM base AS build_client
WORKDIR /workdir
COPY ./crates /workdir/crates
RUN cargo build --manifest-path=crates/tcp_client/Cargo.toml --release

FROM base AS build_server
WORKDIR /workdir
COPY ./crates /workdir/crates
RUN cargo build --manifest-path=crates/tcp_server/Cargo.toml --release

FROM debian:bookworm-slim
WORKDIR /WORKDIR
COPY --from=build_client /workdir/crates/tcp_client/target/release/tcp_client tcp_client
COPY --from=build_server /workdir/crates/tcp_server/target/release/tcp_server tcp_server
