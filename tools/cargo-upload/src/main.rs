use anyhow::{bail, Result};
use clap::{ArgAction, ArgGroup, Parser};
use futures::stream::{FuturesUnordered, StreamExt};
use reqwest::multipart;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Semaphore;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(name = "rust-upload", about = "Upload a file (real or dummy)")]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .args(&["size", "path"])
))]
struct Cli {
    /// Upload in single-threaded mode (dummy only)
    #[arg(long, short = 's', action = ArgAction::SetTrue)]
    single: bool,

    /// Dummy file size, e.g. 100mb, 1g
    #[arg(long)]
    size: Option<String>,

    /// Path to an existing file (uploaded as chunked parts)
    #[arg(long)]
    path: Option<PathBuf>,

    /// Chunk size for uploads (dummy and real), e.g. 8mb
    #[arg(long, default_value = "8mb")]
    chunk_size: String,

    /// Max concurrent uploads (applies to dummy and real)
    #[arg(long, default_value_t = 4)]
    max_concurrent: usize,

    /// Server URL
    #[arg(long, default_value = "http://localhost:8080/upload")]
    server_url: String,
}

#[derive(Serialize, Deserialize)]
struct UploadResponse {
    upload_id: String,
    file_id: String,
}
fn parse_size_bytes(s: &str) -> Result<u64> {
    let s = s.to_lowercase();
    let (digits, unit) = s.chars().partition::<String, _>(|c| c.is_ascii_digit());
    let num: u64 = digits.parse()?;
    Ok(match unit.as_str() {
        "" | "b" => num,
        "k" | "kb" => num * 1024,
        "m" | "mb" => num * 1024 * 1024,
        "g" | "gb" => num * 1024 * 1024 * 1024,
        "t" | "tb" => num * 1024 * 1024 * 1024 * 1024,
        _ => bail!("Unknown unit '{}'", unit),
    })
}

fn pretty_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;
    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

fn sha256_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().iter().map(|b| format!("{:02x}", b)).collect()
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = reqwest::Client::new();
    let chunk_size = parse_size_bytes(&cli.chunk_size)?;
    let max_concurrent = if cli.single { 1 } else { cli.max_concurrent };

    // === Real file mode: --path (chunked like dummy) ===
    if let Some(path) = cli.path {
        let file_size = std::fs::metadata(&path)?.len();
        let total_chunks = file_size.div_ceil(chunk_size);
        let filename = path.file_name().unwrap().to_string_lossy().to_string();
        let content_type = mime_guess::from_path(&path)
            .first_or_octet_stream()
            .essence_str()
            .to_string();

        println!(
            "[*] Uploading real file (chunked): {} ({}, {}, ct={}, chunk={}, chunks={}, concurr={})",
            filename,
            file_size,
            pretty_bytes(file_size),
            &content_type,
            pretty_bytes(chunk_size),
            total_chunks,
            max_concurrent
        );

        let form = multipart::Form::new()
            .text("fileName", filename.clone())
            .text("contentType", content_type.clone())
            .text("token", "todo");

        let start = Instant::now();
        let upload_response: UploadResponse = client
            .post(format!("{}/create", &cli.server_url))
            .multipart(form)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        let upload_id = upload_response.upload_id.trim().to_string();
        let file_id = upload_response.file_id.trim().to_string();
        if upload_id.is_empty() {
            bail!("Empty uploadId from server");
        }
        println!("[✓] uploadId: {}", upload_id);

        // Spawn remaining chunks (2..=total_chunks) — each task re-opens the file and reads its range
        let semaphore = Arc::new(Semaphore::new(max_concurrent));
        let mut tasks = FuturesUnordered::new();

        for chunk_num in 1..=total_chunks {
            let client = client.clone();
            let server_url = cli.server_url.clone();
            let upload_id = upload_id.clone();
            let file_id = file_id.clone();
            let content_type = content_type.clone();
            let semaphore = semaphore.clone();
            let path = path.clone();
            let chunk_size = chunk_size;
            let file_size = file_size;

            tasks.push(tokio::spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();

                let offset = (chunk_num - 1) * chunk_size;
                let this_size = std::cmp::min(chunk_size, file_size - offset);
                let mut buf = vec![0u8; this_size as usize];

                // read this chunk from disk
                let mut rf = File::open(&path)?;
                rf.seek(SeekFrom::Start(offset))?;
                rf.read_exact(&mut buf)?;

                let checksum = sha256_bytes(&buf);

                let form = multipart::Form::new()
                    .text("uploadId", upload_id)
                    .text("fileId", file_id)
                    .text("chunkNumber", chunk_num.to_string())
                    .text("totalChunks", total_chunks.to_string())
                    .text("contentType", content_type)
                    .text("checksum", checksum)
                    .part("chunk", multipart::Part::bytes(buf));

                let response = client.post(&server_url).multipart(form).send().await?;
                if !response.status().is_success() {
                    bail!("Failed to upload chunk {}: {}", chunk_num, response.text().await?);
                }
                Ok::<(), anyhow::Error>(())
            }));
        }

        while let Some(res) = tasks.next().await {
            res??;
        }

        let secs = start.elapsed().as_secs().max(1);
        let bps = file_size / secs;
        let mbps = (file_size * 8) / secs / 1_000_000;

        println!("[✓] Real file upload complete in {}s — {} B/s (~{} Mbps)", secs, bps, mbps);
        return Ok(());
    }

    // === Dummy mode: --size (chunked, optionally concurrent) ===
    let size = parse_size_bytes(cli.size.as_ref().unwrap())?;
    let total_chunks = size.div_ceil(chunk_size);
    let content_type = "application/octet-stream".to_string();

    println!(
        "[*] Dummy upload: {} ({}), chunk={}, chunks={}, concurr={}, ct={}",
        size,
        pretty_bytes(size),
        pretty_bytes(chunk_size),
        total_chunks,
        max_concurrent,
        &content_type
    );

    // first dummy chunk
    let filename = "uploadfile.bin".to_string();

    let form = multipart::Form::new()
        .text("fileName", filename.clone())
        .text("contentType", content_type.clone())
        .text("token", "todo");

    let start = Instant::now();
    let upload_response: UploadResponse = client
        .post(format!("{}/create", &cli.server_url))
        .multipart(form)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    let upload_id = upload_response.upload_id.trim().to_string();
    let file_id = upload_response.file_id.trim().to_string();

    if upload_id.is_empty() {
        bail!("Empty uploadId from server");
    }
    println!("[✓] uploadId: {}", upload_id);

    let semaphore = Arc::new(Semaphore::new(max_concurrent));
    let mut tasks = FuturesUnordered::new();

    for chunk_num in 1..=total_chunks {
        let client = client.clone();
        let server_url = cli.server_url.clone();
        let upload_id = upload_id.clone();
        let filename = file_id.clone();
        let chunk_size = chunk_size;
        let size = size;
        let content_type = content_type.clone();
        let semaphore = semaphore.clone();

        tasks.push(tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();

            let offset = (chunk_num - 1) * chunk_size;
            let this_size = std::cmp::min(chunk_size, size - offset);
            println!("[>] uploading chunk {}/{} (offset={}, size={})",
                chunk_num, total_chunks, offset, pretty_bytes(this_size));
            let buffer = vec![0u8; this_size as usize];
            let hash = sha256_bytes(&buffer);

            let form = multipart::Form::new()
                .text("uploadId", upload_id)
                .text("fileId", filename)
                .text("chunkNumber", chunk_num.to_string())
                .text("totalChunks", total_chunks.to_string())
                .text("contentType", content_type)
                .text("checksum", hash)
                .part("chunk", multipart::Part::bytes(buffer));

            let error = client.post(&server_url).multipart(form).send().await?;
            if !error.status().is_success() {
                bail!("Failed to upload chunk {}: {}", chunk_num, error.text().await?);
            }
            Ok::<(), anyhow::Error>(())
        }));
    }

    while let Some(res) = tasks.next().await {
        res??;
    }

    let secs = start.elapsed().as_secs().max(1);
    let bps = size / secs;
    let mbps = (size * 8) / secs / 1_000_000;

    println!("[✓] Dummy upload complete in {}s — {} B/s (~{} Mbps)", secs, bps, mbps);
    Ok(())
}
