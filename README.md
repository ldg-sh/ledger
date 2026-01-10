# Ledger

A storage proxy system that sits between you and object storage. Adds support for custom upload/download protocols and optimized data transfer. Think of it as Google Drive but faster and more flexible.

## Structure

```
apps/
  backend/    Rust - Main API (file upload/download, S3 proxy)
  web/        Next.js - Web frontend

packages/
  rust/
    ledger-common/            Shared Rust code (errors, responses, config)
    ledger-backend-entity/    SeaORM entities for backend DB
    ledger-backend-migration/ Database migrations for backend

tools/        CLI tools (cargo-upload, cargo-download)
```

## Getting Started

### Prerequisites

- Rust (latest stable)
- Node.js 20+
- pnpm
- PostgreSQL, Redis, S3-compatible storage (we use Railway for dev)

### Setup

1. Clone the repo
2. Install dependencies:
   ```bash
   pnpm install
   ```
3. Copy environment files and fill in your values:
   ```bash
   cp apps/backend/.env.example apps/backend/.env
   cp apps/web/.env.example apps/web/.env
   ```

4. Run migrations:
   ```bash
   ./ledger migrate
   ```

5. Start dev servers:
   ```bash
   ./ledger dev
   ```

## CLI

Everything runs through `./ledger`:

```bash
./ledger dev                    # Start all services (backend, web)
./ledger dev --backend          # Start only backend
./ledger dev --web              # Start only web frontend

./ledger build                  # Build all Rust packages
./ledger check                  # Type-check all Rust packages

./ledger migrate                # Run migrations (default: up)
./ledger migrate status         # Check migration status
./ledger migrate down           # Rollback migrations

./ledger generate               # Generate entities from database
```

## Environment Variables

### Backend (`apps/backend/.env`)

| Variable | Description |
|----------|-------------|
| `POSTGRES_URI` | PostgreSQL connection string |
| `REDIS_URL` | Redis connection string |
| `S3_URL` | S3/MinIO endpoint |
| `S3_BUCKET_NAME` | S3 bucket name |
| `S3_ACCESS_KEY` | S3 access key |
| `S3_SECRET_KEY` | S3 secret key |
| `S3_BUCKET_REGION` | S3 region |

### Web (`apps/web/.env`)

| Variable | Description |
|----------|-------------|
| `API_URL` | Backend API URL (default: http://localhost:8080) |

## Roadmap

- [x] Upload / Download
- [x] Files as CDN
- [x] File delete
- [ ] Authentication (coming soon)
- [ ] File sharing (password/public links)
- [ ] File encryption at rest
