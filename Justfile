set dotenv-load := true
set shell := ["bash", "-uc"]

# Configuration
infra_compose := "docker-compose.infra.yml"
auth_dir := "../ledger-auth"

# Common env with sensible defaults (override in .env)
S3_ACCESS_KEY := env_var_or_default("S3_ACCESS_KEY", "ledger")
S3_SECRET_KEY := env_var_or_default("S3_SECRET_KEY", "ledger-password")
S3_BUCKET_NAME := env_var_or_default("S3_BUCKET_NAME", "ledger")
S3_BUCKET_REGION := env_var_or_default("S3_BUCKET_REGION", "auto")
S3_URL := env_var_or_default("S3_URL", "http://127.0.0.1:9000")

POSTGRES_HOST := env_var_or_default("POSTGRES_HOST", "127.0.0.1")
POSTGRES_PORT := env_var_or_default("POSTGRES_PORT", "5432")
POSTGRES_AUTH_PORT := env_var_or_default("POSTGRES_AUTH_PORT", "5433")
POSTGRES_USER := env_var_or_default("POSTGRES_USER", "postgres")
POSTGRES_PASSWORD := env_var_or_default("POSTGRES_PASSWORD", "postgres")

REDIS_URL := env_var_or_default("REDIS_URL", "redis://127.0.0.1:6379")

AUTH_HTTP_PORT := env_var_or_default("AUTH_HTTP_PORT", "8081")
AUTH_GRPC_PORT := env_var_or_default("AUTH_GRPC_PORT", "50051")
GRPC_AUTH_KEY := env_var_or_default("GRPC_AUTH_KEY", "dev-secret")

# No derived URIs at top-level: build them inline so interpolation works

default:
    @just --list

# ----- Infrastructure (Docker only for deps) -----
infra-up:
    # Starts postgres, postgres-auth, redis, and minio with host ports exposed
    docker compose -f {{infra_compose}} up -d postgres postgres-auth redis minio

infra-down:
    docker compose -f {{infra_compose}} down

infra-logs:
    docker compose -f {{infra_compose}} logs -f --tail=200 postgres postgres-auth redis minio

infra-ps:
    docker compose -f {{infra_compose}} ps

# ----- App: run locally with hot reload -----
ensure-tools:
    if ! command -v cargo-watch >/dev/null 2>&1; then \
    echo "Installing cargo-watch"; \
    cargo install cargo-watch; \
    else \
    echo "cargo-watch already installed"; \
    fi

auth-dev: ensure-tools
    # Runs ledger-auth locally with hot reload
    if [ ! -d {{auth_dir}} ]; then \
      echo "Missing {{auth_dir}}. Clone the repo next to this one."; \
      exit 1; \
    fi
    cd {{auth_dir}} && \
    RESEND_KEY="${RESEND_KEY:-dev-resend}" \
    ADMIN_KEY="${ADMIN_KEY:-admin-dev}" \
    PORT={{AUTH_HTTP_PORT}} \
    POSTGRES_URI="postgresql://{{POSTGRES_USER}}:{{POSTGRES_PASSWORD}}@{{POSTGRES_HOST}}:{{POSTGRES_AUTH_PORT}}/ledger" \
    GRPC_PORT={{AUTH_GRPC_PORT}} \
    GRPC_AUTH_KEY={{GRPC_AUTH_KEY}} \
    cargo watch -- cargo run --bin ledger-auth

ledger-dev: ensure-tools
    # Runs ledger locally with hot reload
    # Force MinIO for dev regardless of .env Backblaze/B2 values
    S3_ACCESS_KEY=ledger \
    S3_SECRET_KEY=ledger-password \
    S3_BUCKET_NAME=ledger \
    S3_BUCKET_REGION=us-east-1 \
    S3_URL=http://127.0.0.1:9000 \
    POSTGRES_URI="postgresql://{{POSTGRES_USER}}:{{POSTGRES_PASSWORD}}@{{POSTGRES_HOST}}:{{POSTGRES_PORT}}/ledger" \
    REDIS_URI={{REDIS_URL}} \
    GRPC_URL="http://127.0.0.1:{{AUTH_GRPC_PORT}}" \
    GRPC_AUTH_KEY={{GRPC_AUTH_KEY}} \
    cargo watch -x 'run'

dev: ensure-tools
    #!/usr/bin/env bash
    # Runs both auth and ledger with hot reload; stops both on exit
    set -euo pipefail
    # Pretty, docker-compose-like colored prefixes
    COLOR_RESET="$(printf '\033[0m')"
    COLOR_AUTH="$(printf '\033[36m')"   # cyan
    COLOR_LEDGER="$(printf '\033[32m')" # green
    WIDTH=12
    printf -v SVC_AUTH   "%-${WIDTH}s" "auth"
    printf -v SVC_LEDGER "%-${WIDTH}s" "ledger"
    PREFIX_AUTH="${COLOR_AUTH}${SVC_AUTH}${COLOR_RESET} | "
    PREFIX_LEDGER="${COLOR_LEDGER}${SVC_LEDGER}${COLOR_RESET} | "
    # Start auth with prefixed logs (portable, no stdbuf)
    (
      cd {{auth_dir}} && \
      RESEND_KEY="${RESEND_KEY:-dev-resend}" \
      ADMIN_KEY="${ADMIN_KEY:-admin-dev}" \
      PORT={{AUTH_HTTP_PORT}} \
      POSTGRES_URI="postgresql://{{POSTGRES_USER}}:{{POSTGRES_PASSWORD}}@{{POSTGRES_HOST}}:{{POSTGRES_AUTH_PORT}}/ledger" \
      GRPC_PORT={{AUTH_GRPC_PORT}} \
      GRPC_AUTH_KEY={{GRPC_AUTH_KEY}} \
      cargo watch -- cargo run --bin ledger-auth \
    ) > >(awk -v p="$PREFIX_AUTH" '{print p $0}') \
      2> >(awk -v p="$PREFIX_AUTH" '{print p $0}' >&2) &
    AUTH_PID=$!
    # Start ledger with prefixed logs (portable, no stdbuf)
    (
      # Force MinIO for dev regardless of .env Backblaze/B2 values
      S3_ACCESS_KEY=ledger \
      S3_SECRET_KEY=ledger-password \
      S3_BUCKET_NAME=ledger \
      S3_BUCKET_REGION=us-east-1 \
      S3_URL=http://127.0.0.1:9000 \
      POSTGRES_URI="postgresql://{{POSTGRES_USER}}:{{POSTGRES_PASSWORD}}@{{POSTGRES_HOST}}:{{POSTGRES_PORT}}/ledger" \
      REDIS_URI={{REDIS_URL}} \
      GRPC_URL="http://127.0.0.1:{{AUTH_GRPC_PORT}}" \
      GRPC_AUTH_KEY={{GRPC_AUTH_KEY}} \
      cargo watch -x 'run' \
    ) > >(awk -v p="$PREFIX_LEDGER" '{print p $0}') \
      2> >(awk -v p="$PREFIX_LEDGER" '{print p $0}' >&2) &
    LEDGER_PID=$!
    trap 'kill $AUTH_PID $LEDGER_PID || true' INT TERM EXIT
    wait $AUTH_PID $LEDGER_PID
