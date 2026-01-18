# Ledger Auth

Authentication service for Ledger using Better Auth + Prisma + Hono.

## Features

- Email/password authentication with verification
- Google OAuth
- Session management with backend cache invalidation
- Server-to-server session validation endpoint

## Setup

1. Copy `.env.example` to `.env` and configure
2. Install dependencies: `bun install`
3. Generate Prisma client: `bun run build`
4. Run migrations: `bunx prisma migrate dev`
5. Start server: `bun run dev`

## API Endpoints

- `POST /api/auth/*` - Better Auth endpoints
- `GET /api/health` - Health check
- `GET /api/validate/session` - Server-to-server session validation

## Session Validation

The Rust backend validates sessions by calling:

```
GET /api/validate/session
Headers:
  X-Session-Token: <user session token>
  X-Server-Secret: <shared server key>
```

Returns user info if valid, 401 if invalid.

## Development

```bash
# Start dev server with hot reload
bun run dev

# Generate Prisma client
bun run build

# Run migrations
bunx prisma migrate dev
```
