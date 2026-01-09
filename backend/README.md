# Ledger

## What is it?
Ledger is a **storage proxy system** that sits between you and object storage.
It adds support for **custom upload/download protocols** and optimized data transfer.
Think of it as *Google Drive* but faster and more flexible.

⚠️ **Note:** This project is currently under construction. The roadmap and features are still being finalized.

## How to host
Download [`docker-compose.yml`](https://github.com/ldg-sh/ledger/blob/main/docker-compose.yml) and place it in an empty directory.
```bash
docker compose up -d
```

## Features for MVP
- [x] Upload
- [x] Download
- [x] Files as CDN
- [x] File delete
- [x] Authenticated access via gRPC
- [ ] Ability to safely share files (password or public links)
- [ ] File encryption at rest (SSE-C AES-256? probably SSE-C and "workspace" specific decryption)

## Configuration
Set the following environment variables (see `.env.example` for a template, if present):

- `S3_BUCKET_NAME`, `S3_ACCESS_KEY`, `S3_SECRET_KEY`, `S3_URL`, `S3_BUCKET_REGION`
- `POSTGRES_URI`
- `GRPC_URL`, `GRPC_AUTH_KEY`
- `REDIS_URL`
