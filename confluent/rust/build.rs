fn main() -> std::io::Result<()> {
    prost_build::compile_protos(
        &["../../protos/protobuf/v1/metrics.proto"],
        &["../../protos/"],
    )?;
    Ok(())
}
