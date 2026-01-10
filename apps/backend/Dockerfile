# Build from repo root: docker build -f apps/backend/Dockerfile .
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
RUN apt-get update && apt-get install -y git protobuf-compiler musl-tools libssl-dev
RUN rustup target add x86_64-unknown-linux-musl

# Clone proto definitions
RUN git clone --depth 1 --branch main https://github.com/ldg-sh/ledger-protobuf proto

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY apps/backend ./apps/backend
COPY packages/rust ./packages/rust
COPY tools ./tools

RUN cargo chef prepare --recipe-path recipe.json --bin ledger

FROM chef AS builder
RUN apt-get update && apt-get install -y protobuf-compiler git musl-tools libssl-dev
RUN rustup target add x86_64-unknown-linux-musl
ENV PROTOC=/usr/bin/protoc

# Clone proto definitions
RUN git clone --depth 1 --branch main https://github.com/ldg-sh/ledger-protobuf proto

COPY --from=planner /app/recipe.json recipe.json

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY apps/backend ./apps/backend
COPY packages/rust ./packages/rust
COPY tools ./tools

RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json --bin ledger

COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin ledger

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/ledger /usr/local/bin/ledger
CMD ["/usr/local/bin/ledger"]
