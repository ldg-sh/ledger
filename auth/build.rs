use std::env;
use std::path::PathBuf;
use tonic_prost_build::configure;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=NULL");
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?).join("proto");
    let proto_file = root.join("auth/auth.proto");

    configure()
        .compile_protos(&[proto_file], &[root])?;
    Ok(())
}
