FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /ledger-auth

FROM chef AS planner
RUN apt-get update && apt-get install -y git protobuf-compiler

RUN git clone --depth 1 --branch main https://github.com/ldg-sh/ledger-protobuf proto

COPY Cargo.toml Cargo.lock ./
COPY entity ./entity
COPY migration ./migration
COPY src ./src
COPY build.rs ./build.rs
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
RUN apt-get update && apt-get install -y protobuf-compiler git
ENV PROTOC=/usr/bin/protoc

ARG PROTO_REPO_URL
ARG PROTO_REPO_BRANCH=main
RUN git clone --depth 1 --branch main https://github.com/ldg-sh/ledger-protobuf proto

COPY --from=planner /ledger-auth/recipe.json recipe.json

COPY Cargo.toml Cargo.lock ./
COPY entity ./entity
COPY migration ./migration
COPY src ./src
COPY build.rs ./build.rs

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin ledger-auth

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /ledger-auth/target/release/ledger-auth /usr/local/bin/ledger-auth
CMD ["/usr/local/bin/ledger-auth"]