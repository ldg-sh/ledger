# Ledger Auth

Ledger Auth is the **custom authentication server** for the Ledger project.
It provides secure token issuance and access control for Ledger.


⚠️ **Note:** This project is currently under active construction. The roadmap and features are still under construction.

## Global features for MVP.
- [x] Upload
- [x] Download
- [x] Files as CDN
- [x] Better error handling.
- [x] Standard response type.
- [x] File delete
- [x] User create, update, and delete
- [ ] Lock files ops behind auth
- [x] Token reset endpoint + email notification
- [ ] Ability to safely share files (password or public scopes)
- [ ] Pluggable RBAC once scope expands again
- [ ] File encryption at rest (SSE-C AES-256? probably SSE-C and "workspace" specific decryption)
- [ ] Team deletion (should email admin with a conf code)

## Auth MVP.
- [x] User create, update, and delete
- [x] Token-based auth for file access (single-tenant)
- [ ] Admin/user roles (future)
