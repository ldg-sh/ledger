FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /ledger

FROM chef AS planner
RUN apt-get update && apt-get install -y git protobuf-compiler

RUN git clone --depth 1 --branch main https://github.com/ldg-sh/ledger-protobuf proto

COPY Cargo.toml Cargo.lock ./
COPY entity ./entity
COPY migration ./migration
COPY src ./src
COPY build.rs ./build.rs
COPY tools ./tools

RUN cargo chef prepare --recipe-path recipe.json --bin ledger

FROM chef AS builder
RUN apt-get update && apt-get install -y protobuf-compiler git
ENV PROTOC=/usr/bin/protoc

ARG PROTO_REPO_URL
ARG PROTO_REPO_BRANCH=main
RUN git clone --depth 1 --branch main https://github.com/ldg-sh/ledger-protobuf proto

COPY --from=planner /ledger/recipe.json recipe.json

COPY Cargo.toml Cargo.lock ./
COPY entity ./entity
COPY migration ./migration
COPY src ./src
COPY build.rs ./build.rs
COPY tools ./tools

RUN cargo chef cook --release --recipe-path recipe.json --bin ledger

COPY . .
RUN cargo build --release --bin ledger

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /ledger/target/release/ledger /usr/local/bin/ledger
CMD ["/usr/local/bin/ledger"]