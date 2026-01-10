# Ledger Backend

The storage proxy API that sits between you and object storage. Handles file upload/download, S3 proxying, and optimized data transfer.

## Features

- [x] Upload / Download
- [x] Files as CDN
- [x] File delete
- [ ] Authentication (coming soon)
- [ ] File sharing (password/public links)
- [ ] File encryption at rest

## Configuration

Set the following environment variables (see `.env.example`):

- `S3_BUCKET_NAME`, `S3_ACCESS_KEY`, `S3_SECRET_KEY`, `S3_URL`, `S3_BUCKET_REGION`
- `POSTGRES_URI`
- `REDIS_URL`

## Development

From the monorepo root:

```bash
./ledger dev --backend    # Run backend only
./ledger check            # Type-check
./ledger fmt              # Format code
./ledger migrate          # Run migrations
```
