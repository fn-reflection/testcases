fn main() -> std::io::Result<()> {
    prost_build::Config::new().compile_protos(
        &["../../protos/protobuf/v1/metrics.proto"],
        &["../../protos/"],
    )?;
    Ok(())
}
