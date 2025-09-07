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
- [x] User create, update, and delete
- [ ] Lock files ops behind auth
- [ ] Team based auth for file access (even if solo)
- [ ] Team based admin controls, add/remove users (even if solo)
- [ ] Ability to safely share files (team member, password, or public)
- [ ] Bucket folder structure per team; team names must be unique
- [ ] File encryption at rest (SSE-C AES-256? probably SSE-C and "workspace" specific decryption)
