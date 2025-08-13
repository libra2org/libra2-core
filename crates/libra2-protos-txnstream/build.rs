fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .format(false)
        .compile(&["proto/txnstream/v1/txnstream.proto"], &["proto"])?;
    Ok(())
}
