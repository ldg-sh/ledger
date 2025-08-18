use anyhow::{bail, Result};
use clap::{ArgAction, Parser};
use futures::stream::{FuturesUnordered, StreamExt};
use reqwest::multipart;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Semaphore;

#[derive(Parser, Debug)]
#[command(name = "rust-upload")]
#[command(about = "Upload a dummy file in chunks")]
struct Cli {
    /// Upload in single-threaded mode
    #[arg(long, short = 's', action = ArgAction::SetTrue)]
    single: bool,

    /// Total file size, e.g. 100mb, 1g
    size: String,

    /// Chunk size, e.g. 10mb
    #[arg(default_value = "5mb")]
    chunk_size: String,

    /// Max concurrent uploads
    #[arg(default_value_t = 4)]
    max_concurrent: usize,

    /// Server URL
    #[arg(default_value = "http://localhost:8080/upload")]
    server_url: String,
}

fn parse_size_bytes(s: &str) -> Result<u64> {
    let s = s.to_lowercase();
    let mut digits = String::new();
    let mut unit = String::new();
    for c in s.chars() {
        if c.is_ascii_digit() {
            digits.push(c);
        } else {
            unit.push(c);
        }
    }
    let num: u64 = digits.parse()?;
    let bytes = match unit.as_str() {
        "" | "b" => num,
        "k" | "kb" => num * 1024,
        "m" | "mb" => num * 1024 * 1024,
        "g" | "gb" => num * 1024 * 1024 * 1024,
        "t" | "tb" => num * 1024 * 1024 * 1024 * 1024,
        _ => bail!("Unknown unit '{}'", unit),
    };
    Ok(bytes)
}

fn pretty_bytes(bytes: u64) -> String {
    if bytes >= 1024 * 1024 * 1024 {
        format!("{} GB", bytes / (1024 * 1024 * 1024))
    } else if bytes >= 1024 * 1024 {
        format!("{} MB", bytes / (1024 * 1024))
    } else if bytes >= 1024 {
        format!("{} KB", bytes / 1024)
    } else {
        format!("{} B", bytes)
    }
}

fn sha256_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().iter().map(|b| format!("{:02x}", b)).collect()
}

fn make_file(path: &PathBuf, size: u64) -> Result<()> {
    let mut f = File::create(path)?;
    f.seek(SeekFrom::Start(size))?;
    f.write_all(&[0])?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let file_size = parse_size_bytes(&cli.size)?;
    let chunk_size = parse_size_bytes(&cli.chunk_size)?;
    let max_concurrent = if cli.single { 1 } else { cli.max_concurrent };
    let total_chunks = (file_size + chunk_size - 1) / chunk_size;

    let tmpfile = std::env::temp_dir().join("uploadfile.bin");

    println!(
        "[*] Generating file: {} ({}) → {:?}",
        pretty_bytes(file_size),
        file_size,
        tmpfile
    );
    make_file(&tmpfile, file_size)?;

    let actual_size = std::fs::metadata(&tmpfile)?.len();
    if actual_size - 1 != file_size {
        eprintln!(
            "[!] Warning: Created size = {} (expected {})",
            actual_size, file_size
        );
    }

    let filename = tmpfile.file_name().unwrap().to_string_lossy();
    println!("[*] Server: {}", cli.server_url);
    println!("[*] Filename: {}", filename);
    println!("[*] Total size: {} bytes ({})", file_size, pretty_bytes(file_size));
    println!(
        "[*] Chunk size: {} bytes ({})",
        chunk_size,
        pretty_bytes(chunk_size)
    );
    println!("[*] Total chunks: {}", total_chunks);
    println!("[*] Max concurrency: {}", max_concurrent);
    println!();

    let client = reqwest::Client::new();
    let start = Instant::now();

    println!("[→] Uploading chunk 1/{} (init)…", total_chunks);
    let mut first_chunk = vec![0u8; chunk_size as usize];

    File::open(&tmpfile)?.read_exact(&mut first_chunk)?;

    let hash = sha256_bytes(&first_chunk);

    let form = multipart::Form::new()
        .text("fileName", filename.to_string())
        .text("chunkNumber", "1")
        .text("totalChunks", total_chunks.to_string())
        .text("checksum", hash.clone())
        .part("chunk", multipart::Part::bytes(first_chunk));
    let resp = client
        .post(&cli.server_url)
        .multipart(form)
        .send()
        .await?
        .text()
        .await?;
    let upload_id = resp.trim();
    if upload_id.is_empty() {
        bail!("Empty uploadId from server");
    }
    println!("[✓] Received uploadId: {}", upload_id);
    println!();

    let semaphore = Arc::new(Semaphore::new(max_concurrent));
    let mut tasks = FuturesUnordered::new();

    for chunk_num in 2..=total_chunks {
        let client = client.clone();
        let semaphore = semaphore.clone();
        let file_path = tmpfile.clone();
        let server_url = cli.server_url.clone();
        let upload_id = upload_id.to_string();

        tasks.push(tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            let offset = (chunk_num - 1) * chunk_size;
            let size = std::cmp::min(chunk_size, file_size - offset);
            let mut buffer = vec![0u8; size as usize];
            File::open(&file_path)?.seek(SeekFrom::Start(offset))?;
            File::open(&file_path)?.read_exact(&mut buffer)?;
            let hash = sha256_bytes(&buffer);

            let form = multipart::Form::new()
                .text("uploadId", upload_id)
                .text("fileName", "uploadfile.bin".to_string())
                .text("chunkNumber", chunk_num.to_string())
                .text("totalChunks", total_chunks.to_string())
                .text("checksum", hash.clone())
                .part("chunk", multipart::Part::bytes(buffer));

            println!("[→] Uploading chunk {}/{}…", chunk_num, total_chunks);
            client.post(&server_url).multipart(form).send().await?;
            Ok::<(), anyhow::Error>(())
        }));
    }

    while let Some(res) = tasks.next().await {
        res??; //dafuq are there two ?????s here
    }

    let duration = start.elapsed().as_secs().max(1);
    let bytes_per_sec = file_size / duration;
    let mbps = (file_size * 8) / duration / 1_000_000;

    println!();
    println!("[✓] Upload complete!");
    println!("[*] Elapsed: {}s", duration);
    println!(
        "[*] Avg throughput: {} B/s (~{} Mbps)",
        bytes_per_sec, mbps
    );

    std::fs::remove_file(tmpfile)?;
    println!("[*] Temp file removed");

    Ok(())
}