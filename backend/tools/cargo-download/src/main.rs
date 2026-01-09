use anyhow::{anyhow, bail, Result};
use clap::Parser;
use futures::stream::{FuturesUnordered, StreamExt};
use reqwest::multipart;
use serde_json::Value;
use std::sync::Arc;
use std::{cmp::min, path::PathBuf};
use tokio::fs::{self, OpenOptions};
use tokio::io::SeekFrom;
use tokio::io::{AsyncSeekExt, AsyncWriteExt};
use tokio::sync::Semaphore;

#[derive(Parser, Debug)]
#[command(
    name = "rust-download",
    about = "Download a file in ranged parts using your /download endpoints"
)]
struct Cli {
    /// Remote file name (server-side object key)
    #[arg(long)]
    file_name: String,

    /// Base server URL for the download scope (has /metadata and root)
    #[arg(long, default_value = "http://localhost:8080/download")]
    server_url: String,

    /// Part size, e.g. 8mb, 16mb
    #[arg(long, default_value = "8mb")]
    part_size: String,

    /// Max concurrent part downloads
    #[arg(long, default_value_t = 6)]
    max_concurrent: usize,

    /// Output path (defaults to file_name)
    #[arg(long)]
    output: Option<PathBuf>,
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
        _ => bail!("Unknown unit '{}'", unit),
    })
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = reqwest::Client::new();

    // 1) HEAD-equivalent: fetch metadata via GET /metadata (multipart form with fileName)
    let meta_url = format!("{}/metadata", cli.server_url);
    let meta_form = multipart::Form::new().text("fileId", cli.file_name.clone());

    let resp = client
        .get(&meta_url)
        .multipart(meta_form)
        .send()
        .await?
        .error_for_status()?;

    let body = resp.text().await?;
    let meta: Value = serde_json::from_str(&body)?;

    // Try common keys: content_length / ContentLength / size / length
    let size = meta
        .get("content_size")
        .and_then(|v| v.as_u64()) // your server
        .or_else(|| meta.get("content_length").and_then(|v| v.as_u64()))
        .or_else(|| meta.get("ContentLength").and_then(|v| v.as_u64()))
        .or_else(|| meta.get("size").and_then(|v| v.as_u64()))
        .or_else(|| meta.get("length").and_then(|v| v.as_u64()))
        .ok_or_else(|| anyhow!("metadata missing file size; got: {}", meta))?;

    let ct = meta
        .get("mime")
        .and_then(|v| v.as_str()) // your server
        .or_else(|| meta.get("content_type").and_then(|v| v.as_str()))
        .or_else(|| meta.get("ContentType").and_then(|v| v.as_str()))
        .unwrap_or("application/octet-stream");

    let out_path = cli.output.unwrap_or_else(|| PathBuf::from(&cli.file_name));
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent).await.ok();
    }

    // 2) Pre-allocate output file
    let f = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(&out_path)
        .await?;
    f.set_len(size).await?;

    // 3) Build ranges
    let part = parse_size_bytes(&cli.part_size)?;
    let total_parts = ((size + part - 1) / part) as usize;

    println!(
        "[*] Downloading '{}' -> '{}' ({} bytes, ct='{}'), parts={}, part_size={}",
        cli.file_name,
        out_path.display(),
        size,
        ct,
        total_parts,
        &cli.part_size
    );

    // 4) Download parts concurrently
    let sem = Arc::new(Semaphore::new(cli.max_concurrent));
    let mut tasks = FuturesUnordered::new();

    for i in 0..total_parts {
        let start = (i as u64) * part;
        let end = min(size - 1, start + part - 1);

        let client = client.clone();
        let server_url = cli.server_url.clone();
        let file_name = cli.file_name.clone();
        let out_path = out_path.clone();
        let sem = sem.clone();

        tasks.push(tokio::spawn(async move {
            let _permit = sem.acquire().await.unwrap();

            // Call GET /download with multipart form: fileName, rangeStart, rangeEnd
            let form = multipart::Form::new()
                .text("fileId", file_name)
                .text("rangeStart", start.to_string())
                .text("rangeEnd", end.to_string());

            let bytes = client
                .get(&server_url)
                .multipart(form)
                .send()
                .await?
                .error_for_status()?
                .bytes()
                .await?;

            // Write to file at offset
            let mut fh = OpenOptions::new().write(true).open(&out_path).await?;
            fh.seek(SeekFrom::Start(start)).await?;
            fh.write_all(&bytes).await?;
            Ok::<(), anyhow::Error>(())
        }));
    }

    let mut done = 0usize;
    while let Some(res) = tasks.next().await {
        res??;
        done += 1;
        if done % 10 == 0 || done == total_parts {
            println!("[=] {}/{} parts", done, total_parts);
        }
    }

    println!("[✓] Download complete: {}", out_path.display());
    Ok(())
}
