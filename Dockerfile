FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
RUN cargo install cargo-chef
WORKDIR /ledger

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
RUN apt-get update && apt-get install -y protobuf-compiler
ENV PROTOC=/usr/bin/protoc

COPY --from=planner /ledger/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin ledger

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /ledger/target/release/ledger /usr/local/bin/ledger
CMD ["/usr/local/bin/ledger"]
