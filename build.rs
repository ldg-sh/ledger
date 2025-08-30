use tonic_prost_build::configure;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    configure()
        .compile_protos(
            &[
                "proto/auth/auth.proto"
            ],
            &["proto"],
        )?;
    Ok(())
}
