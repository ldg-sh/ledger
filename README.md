Ledger

Efficient, resumable, chunked file upload and download service in Rust. Ledger exposes an HTTP API for large-file uploads to any S3‑compatible storage (MinIO, Backblaze B2 S3 API, AWS S3, etc.) and persists file metadata in PostgreSQL using SeaORM.


Architecture
- Server: `actix-web` service on port `8080`.
- Storage: S3 client configured via endpoint, access/secret, region, bucket.
- DB: PostgreSQL connection auto‑migrated at startup via `migration` crate.
- Workspace: multi‑crate repo with `entity`, `migration`, and `tools/*`.

Quick Start
1) Prerequisites
- Rust toolchain (stable) and Cargo
- PostgreSQL 13+ with a database you can connect to
- S3‑compatible storage (MinIO/Backblaze/AWS)

2) Configure environment
Create a `.env` at the repo root:

```
POSTGRES_URI=postgres://postgres:postgres@localhost:5432/postgres
S3_URL=url
S3_BUCKET_REGION=us-east-1
S3_BUCKET_NAME=ledger
S3_ACCESS_KEY=minioadmin
S3_SECRET_KEY=minioadmin
```

Notes
- The service binds on `::`:8080. Set `RUST_LOG=info` for standard logs.
- `S3_URL` can be a custom endpoint (e.g., MinIO or Backblaze S3 API).

3) Run the server

```
RUST_LOG=info cargo run
```

On boot, the app connects to PostgreSQL and applies pending migrations.

HTTP API
- POST `/upload/create`
  - Multipart form fields:
    - `fileName`: Original file name to record.
    - `contentType`: MIME type to store with the S3 object.
  - Returns JSON: `{ "upload_id": "…", "file_id": "…" }`.
  - Important: subsequent chunk uploads must use `file_id` as the object key.

- POST `/upload`
  - Multipart form fields (one chunk per request):
    - `uploadId`: The ID returned by `/upload/create`.
    - `fileName`: Must be the `file_id` returned by `/upload/create`.
    - `chunkNumber`: 1‑based part index.
    - `totalChunks`: total number of parts for the file.
    - `checksum`: hex‑encoded SHA‑256 of the chunk bytes.
    - `chunk`: the raw bytes (file part) as form file field.
  - Behavior: server validates checksum (translates to S3’s base64 SHA‑256), uploads part, and when all parts are present it completes the multipart upload and updates the DB record.

- GET `/download/metadata?fileName=<file_id>`
  - Returns JSON: `{ content_size: number, mime: string, metadata: {…} }`.

- GET `/download?fileName=<file_id>&rangeStart=<u64>&rangeEnd=<u64>`
  - Returns HTTP 206 Partial Content with the requested byte range.
  - Response headers include `Accept-Ranges: bytes` and `Content-Type`.

- GET `/download/view/{file_id}`
  - Streams the entire object inline (useful for browsers/players).

- GET `/download/list/all`
  - Returns JSON array of stored files with `file_id`, `file_name`, `file_size`.

cURL Examples
- Initialize upload
```
curl -sS -F fileName=example.mp4 \
         -F contentType=video/mp4 \
  http://localhost:8080/upload/create
```

- Upload a chunk (replace vars with real values)
```
CHUNK=chunk_001.bin
CHECKSUM=$(shasum -a 256 "$CHUNK" | awk '{print $1}')
curl -sS -F uploadId=$UPLOAD_ID \
         -F fileName=$FILE_ID \
         -F chunkNumber=1 \
         -F totalChunks=$TOTAL \
         -F checksum=$CHECKSUM \
         -F chunk=@"$CHUNK" \
  http://localhost:8080/upload
```

- Query metadata
```
curl -sS "http://localhost:8080/download/metadata?fileName=$FILE_ID"
```

- Download a range (first 1 MiB)
```
curl -G -o part1.bin \
  --data-urlencode fileName=$FILE_ID \
  --data-urlencode rangeStart=0 \
  --data-urlencode rangeEnd=1048575 \
  http://localhost:8080/download
```

- Stream full file
```
curl -L -o output.bin http://localhost:8080/download/view/$FILE_ID
```

CLI Tools
Two helper binaries live under `tools/`.

- Upload utility (`tools/cargo-upload`)
  - Real file mode:
    ```
    cargo run -p cargo-upload -- --path /path/to/file --chunk-size 8mb --max-concurrent 4 --server-url http://localhost:8080/upload
    ```
  - Dummy data mode:
    ```
    cargo run -p cargo-upload -- --size 1g --chunk-size 8mb --max-concurrent 4
    ```

- Download utility (`tools/cargo-download`)
```
cargo run -p cargo-download -- --file-name $FILE_ID --part-size 8mb --max-concurrent 6 --server-url http://localhost:8080/download
```

Data Model
- Table `file` (managed by SeaORM migrations):
  - `id` (string): the S3 object key, equals `file_id` from `/upload/create`.
  - `file_name` (string): original filename as provided by the client.
  - `file_owner_id` (string): reserved for future auth/tenancy.
  - `upload_id` (string): S3 multipart upload ID.
  - `file_size` (bigint): final size after completion.
  - `created_at` (timestamptz): insertion time.
  - `upload_completed` (boolean): true when multipart is finalized.

Configuration Details
- Env vars are read at startup; missing ones cause a panic. For production, set them in the process environment instead of `.env`.
- Multipart body limit is configured to ~1 GB per request; tune in `main.rs` if needed:
  - `MultipartFormConfig::default().total_limit(1000 * 1024 * 1024)`

Directory Layout
- `src/` server code (routes, modules)
- `src/routes/` HTTP handlers (`/upload`, `/download`)
- `src/modules/s3/` S3 integration (initiate, upload parts, complete, get/stream)
- `src/modules/postgres/` DB connection + queries
- `entity/` SeaORM models
- `migration/` DB migrations and CLI
- `tools/` helper clients for upload/download

Development Tips
- Run with `RUST_LOG=debug` to trace chunk progress and S3 operations.
- Use MinIO locally for S3 testing; make sure the bucket exists.
- The server expects chunk checksums as hex SHA‑256; the service converts to base64 for S3.

Roadmap
- Authentication and per‑user ownership
- Resume discovery and idempotent chunk re‑send
- Soft delete and lifecycle policies
- Presigned upload/download flows

Contributing
Issues and PRs are welcome. Please include a clear description, reproduction steps, and tests where applicable. Before submitting large changes, open an issue to discuss direction.

License
ADD ME
