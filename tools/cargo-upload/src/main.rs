use anyhow::{bail, Context, Result};
use clap::Parser;
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(
    name = "cargo-upload",
    about = "Dev helper mirroring the /upload route"
)]
struct Cli {
    /// Path to the file that should be uploaded
    #[arg(long)]
    file: PathBuf,

    /// Remote file path (server-side object key)
    #[arg(long, default_value = "/")]
    path: PathBuf,

    /// Override the detected content type
    #[arg(long)]
    content_type: Option<String>,

    /// Chunk size to use when splitting the file, e.g. 8mb, 32mb
    #[arg(long, default_value = "8mb")]
    chunk_size: String,

    /// Server origin (without the /upload suffix)
    #[arg(long, default_value = "http://localhost:8080")]
    server: String,
}

#[derive(Debug, Serialize, Deserialize)]
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
        _ => bail!("unknown unit '{unit}'"),
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
        format!("{bytes} B")
    }
}

fn sha256_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher
        .finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let auth_token =
        env::var("TOKEN").context("TOKEN env var must be set for Authorization bearer token")?;
    let chunk_size = parse_size_bytes(&cli.chunk_size)?;
    if chunk_size == 0 {
        bail!("chunk size must be greater than zero");
    }

    let file_path = cli.file.clone();
    let file_metadata = std::fs::metadata(&file_path)
        .with_context(|| format!("failed to read metadata for '{}'", file_path.display()))?;
    let file_size = file_metadata.len();
    if file_size == 0 {
        bail!("file '{}' is empty", file_path.display());
    }

    let file_name = file_path
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.to_string())
        .context("could not determine file name")?;

    let content_type = cli.content_type.clone().unwrap_or_else(|| {
        mime_guess::from_path(&file_path)
            .first_or_octet_stream()
            .essence_str()
            .to_string()
    });
    let total_chunks = file_size.div_ceil(chunk_size);
    if total_chunks == 0 {
        bail!("calculated zero chunks for the upload");
    }
    if total_chunks > u32::MAX as u64 {
        bail!("chunk count {total_chunks} exceeds the u32 limit expected by the API");
    }

    let server = cli.server.trim_end_matches('/');
    let client = reqwest::Client::new();
    let mut file = File::open(&file_path)
        .with_context(|| format!("failed to open '{}'", file_path.display()))?;

    println!("[*] Preparing upload");
    println!(
        "    file: {} ({} bytes, {})",
        file_name,
        file_size,
        pretty_bytes(file_size)
    );
    println!(
        "    chunk size: {} ({} chunks)",
        pretty_bytes(chunk_size),
        total_chunks
    );
    println!("    content-type: {}", content_type);
    println!("    route: {}/upload/<file_id>", server);

    let create_url = format!("{}/upload/create", server);
    let start = Instant::now();

    let create_form = multipart::Form::new()
        .text("fileName", file_name.clone())
        .text("contentType", content_type.clone())
        .text("path", cli.path.to_string_lossy().to_string());

    let UploadResponse { upload_id, file_id } = client
        .post(&create_url)
        .bearer_auth(&auth_token)
        .multipart(create_form)
        .send()
        .await?
        .error_for_status()
        .context("create upload request failed")?
        .json()
        .await
        .context("failed to decode create upload response")?;

    println!("    upload id: {}", upload_id);
    println!("    file id: {}", file_id);

    let chunk_url = format!("{}/upload/{}", server, file_id);
    let mut uploaded_bytes = 0u64;

    for chunk_number in 1..=total_chunks {
        let mut buffer = vec![
            0u8;
            if chunk_number == total_chunks {
                (file_size - uploaded_bytes) as usize
            } else {
                chunk_size as usize
            }
        ];
        file.read_exact(&mut buffer).with_context(|| {
            format!(
                "failed to read chunk {} for '{}'",
                chunk_number,
                file_path.display()
            )
        })?;

        uploaded_bytes += buffer.len() as u64;
        let checksum = sha256_bytes(&buffer);

        let form = multipart::Form::new()
            .text("uploadId", upload_id.clone())
            .text("checksum", checksum)
            .text("chunkNumber", (chunk_number as u32).to_string())
            .text("totalChunks", (total_chunks as u32).to_string())
            .text("path", cli.path.to_string_lossy().to_string())
            .part("chunk", multipart::Part::bytes(buffer));

        client
            .post(&chunk_url)
            .bearer_auth(&auth_token)
            .multipart(form)
            .send()
            .await?
            .error_for_status()
            .with_context(|| format!("chunk {chunk_number} failed"))?;

        let percent = (uploaded_bytes as f64 / file_size as f64) * 100.0;
        println!(
            "[>] chunk {}/{} uploaded ({:.1}% / {})",
            chunk_number,
            total_chunks,
            percent,
            pretty_bytes(uploaded_bytes)
        );
    }

    let elapsed = start.elapsed();
    let secs = elapsed.as_secs_f64();
    let bytes_per_second = file_size as f64 / secs.max(0.001);
    let mbps = (file_size as f64 * 8.0) / secs.max(0.001) / 1_000_000.0;

    println!(
        "[✓] upload complete in {:.2}s — {:.0} B/s (~{:.1} Mbps)",
        secs, bytes_per_second, mbps
    );

    Ok(())
}
