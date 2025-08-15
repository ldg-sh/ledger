#!/bin/bash
# upload.sh — create a dummy file of a given size and upload it in chunks.
# Usage:
#   ./upload.sh [--single|-s] <size> <server_url> [chunk_size] [max_concurrent]
# Examples:
#   ./upload.sh 100mb http://localhost:3000/upload
#   ./upload.sh --single 1g http://localhost:3000/upload 50mb
#
# Notes:
# - <size> accepts: b, k|kb, m|mb, g|gb, t|tb (case-insensitive). e.g., 10mb, 1g.
# - Default CHUNK_SIZE = 10mb, MAX_CONCURRENT = 4 (or 1 with --single).
# - Compatible with macOS bash 3.2 (no `wait -n` used).
# - Server must accept multipart form fields described below.

set -euo pipefail

# --------- Helpers ---------
lower() { echo "$1" | tr '[:upper:]' '[:lower:]'; }
die() { echo "Error: $*" >&2; exit 1; }
need() { command -v "$1" >/dev/null 2>&1 || die "Required command '$1' not found"; }

# Parse human size like "100mb" -> bytes
parse_size_bytes() {
  local in="$(lower "$1")"
  if [[ "$in" =~ ^([0-9]+)(b|kb|k|mb|m|gb|g|tb|t)?$ ]]; then
    local num="${BASH_REMATCH[1]}"
    local unit="${BASH_REMATCH[2]:-b}"
    case "$unit" in
      b)    echo "$num" ;;
      k|kb) echo $(( num * 1024 )) ;;
      m|mb) echo $(( num * 1024 * 1024 )) ;;
      g|gb) echo $(( num * 1024 * 1024 * 1024 )) ;;
      t|tb) echo $(( num * 1024 * 1024 * 1024 * 1024 )) ;;
      *) die "Unknown unit: $unit" ;;
    esac
  else
    die "Invalid size format: '$1' (examples: 10mb, 1g, 500m)"
  fi
}

# Create sparse/dummy file of given byte size into given path
make_file() {
  local bytes="$1"
  local path="$2"
  if command -v mkfile >/dev/null 2>&1; then
    if (( bytes % (1024*1024) == 0 )); then
      local mb=$(( bytes / (1024*1024) ))
      echo "[*] mkfile -n ${mb}m $path"
      mkfile -n "${mb}m" "$path" >/dev/null
    else
      echo "[*] dd (mkfile not exact): creating $bytes bytes"
      dd if=/dev/zero of="$path" bs=1 count="$bytes" status=none
    fi
  elif command -v fallocate >/dev/null 2>&1; then
    echo "[*] fallocate -l $bytes $path"
    fallocate -l "$bytes" "$path"
  else
    local mib=$(( bytes / (1024*1024) ))
    local rem=$(( bytes % (1024*1024) ))
    if (( mib > 0 )); then
      echo "[*] dd: ${mib} MiB"
      dd if=/dev/zero of="$path" bs=1M count="$mib" status=none
    fi
    if (( rem > 0 )); then
      echo "[*] dd remainder: ${rem} bytes"
      dd if=/dev/zero of="$path" bs=1 count="$rem" oflag=append conv=notrunc status=none
    fi
  fi
}

# Human pretty for bytes
pretty_bytes() {
  local b=$1
  local unit="B"
  local val=$b
  if (( b >= 1024 )); then val=$(( b/1024 )); unit="KB"; fi
  if (( b >= 1024*1024 )); then val=$(( b/(1024*1024) )); unit="MB"; fi
  if (( b >= 1024*1024*1024 )); then val=$(( b/(1024*1024*1024) )); unit="GB"; fi
  echo "${val} ${unit}"
}

# --------- Flags ---------
SINGLE=0
if [[ "${1:-}" == "--single" || "${1:-}" == "-s" ]]; then
  SINGLE=1
  shift
fi

# --------- Args ---------
SIZE_ARG="${1:-}"
SERVER_URL="${2:-}"
CHUNK_ARG="${3:-10mb}"
MAX_CONCURRENT="${4:-4}"
if (( SINGLE == 1 )); then MAX_CONCURRENT=1; fi

if [[ -z "$SIZE_ARG" || -z "$SERVER_URL" ]]; then
  echo "Usage: $0 [--single|-s] <size> <server_url> [chunk_size] [max_concurrent]"
  echo "Examples:"
  echo "  $0 100mb http://localhost:3000/upload"
  echo "  $0 --single 1g http://localhost:3000/upload 50mb"
  exit 1
fi

need curl
need dd
need stat

FILESIZE_BYTES="$(parse_size_bytes "$SIZE_ARG")"
CHUNK_SIZE_BYTES="$(parse_size_bytes "$CHUNK_ARG")"

# --------- Prepare temp file ---------
TMPFILE="$(mktemp -t uploadfile.XXXXXX.bin)"
trap 'rm -f "$TMPFILE"' EXIT

echo "[*] Generating file: $(pretty_bytes "$FILESIZE_BYTES") ($FILESIZE_BYTES bytes) → $TMPFILE"
make_file "$FILESIZE_BYTES" "$TMPFILE"

# Verify size
FILESIZE=$(( $(stat -f%z "$TMPFILE" 2>/dev/null || stat -c%s "$TMPFILE") ))
[[ "$FILESIZE" -eq "$FILESIZE_BYTES" ]] || echo "[!] Warning: Created size = $FILESIZE (expected $FILESIZE_BYTES)"

FILENAME="$(basename "$TMPFILE")"
TOTAL_CHUNKS=$(( (FILESIZE + CHUNK_SIZE_BYTES - 1) / CHUNK_SIZE_BYTES ))

echo "[*] Server: $SERVER_URL"
echo "[*] Filename: $FILENAME"
echo "[*] Total size: $FILESIZE bytes ($(pretty_bytes "$FILESIZE"))"
echo "[*] Chunk size: $CHUNK_SIZE_BYTES bytes ($(pretty_bytes "$CHUNK_SIZE_BYTES"))"
echo "[*] Total chunks: $TOTAL_CHUNKS"
echo "[*] Max concurrency: $MAX_CONCURRENT"
echo

# --------- Upload ---------
SECONDS=0

# First chunk: get uploadId
echo "[→] Uploading chunk 1/$TOTAL_CHUNKS (init)…"
RESPONSE=$( dd if="$TMPFILE" bs="$CHUNK_SIZE_BYTES" skip=0 count=1 2>/dev/null | \
  curl -sS -f \
       -F "fileName=$FILENAME" \
       -F "chunkNumber=1" \
       -F "totalChunks=$TOTAL_CHUNKS" \
       -F "chunk=@-" \
       "$SERVER_URL" \
)
UPLOAD_ID="$RESPONSE"
[[ -n "$UPLOAD_ID" ]] || die "Empty uploadId from server"
echo "[✓] Received uploadId: $UPLOAD_ID"
echo

upload_chunk() {
  local chunk_num="$1"
  local skip_num=$(( chunk_num - 1 ))
  printf "[→] Uploading chunk %d/%d…\n" "$chunk_num" "$TOTAL_CHUNKS"
  dd if="$TMPFILE" bs="$CHUNK_SIZE_BYTES" skip="$skip_num" count=1 2>/dev/null | \
    curl -sS -f \
         -F "uploadId=$UPLOAD_ID" \
         -F "fileName=$FILENAME" \
         -F "chunkNumber=$chunk_num" \
         -F "totalChunks=$TOTAL_CHUNKS" \
         -F "chunk=@-" \
         "$SERVER_URL" >/dev/null
}

# Launch chunks 2..N in batches (macOS bash 3.2 safe)
i=2
while (( i <= TOTAL_CHUNKS )); do
  launched=0
  while (( launched < MAX_CONCURRENT && i <= TOTAL_CHUNKS )); do
    upload_chunk "$i" &
    (( i++ ))
    (( launched++ ))
  done
  wait
done

duration=$SECONDS
(( duration == 0 )) && duration=1
bytes_per_sec=$(( FILESIZE / duration ))
mbps=$(( (FILESIZE * 8) / duration / 1000000 ))

echo
echo "[✓] Upload complete!"
echo "[*] Elapsed: ${duration}s"
echo "[*] Avg throughput: ${bytes_per_sec} B/s (~${mbps} Mbps)"
echo "[*] Temp file removed: $TMPFILE"