use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use anyhow::{bail, Context, Result};
use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(name = "cargo-upload", bin_name = "cargo-upload")]
#[command(about = "Proxy to run the project's upload.sh script")]
struct Cli {
    /// Upload in single-threaded mode
    #[arg(long, short = 's', action = ArgAction::SetTrue)]
    single: bool,

    /// Total file size, e.g. 100mb, 1g
    size: String,

    /// Chunk size, e.g. 10mb
    #[arg(default_value = "10mb")]
    chunk_size: String,

    /// Max concurrent uploads
    #[arg(default_value_t = 4)]
    max_concurrent: usize,
}

fn resolve_repo_root() -> Result<PathBuf> {
    // Try to find the repository root by walking up until we find Cargo.toml that matches workspace
    let mut dir = env::current_dir()?;
    for _ in 0..10 {
        let candidate = dir.join("upload.sh");
        if candidate.is_file() {
            return Ok(dir);
        }
        if !dir.pop() { break; }
    }
    bail!("could not locate upload.sh in current or parent directories");
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let root = resolve_repo_root()?;
    let script = root.join("upload.sh");

    let mut args: Vec<String> = Vec::new();
    if cli.single { args.push("--single".into()); }
    args.push(cli.size);
    args.push("http://localhost:8080/upload".into());
    args.push(cli.chunk_size);
    args.push(cli.max_concurrent.to_string());

    let status = Command::new(&script)
        .args(&args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| format!("failed to execute {}", script.display()))?;

    if !status.success() {
        bail!("upload.sh exited with status {:?}", status.code());
    }
    Ok(())
}
