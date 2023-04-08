fn main() -> std::io::Result<()> {
    prost_build::compile_protos(
        &["../../protos/protobuf/v1/migrate.proto"],
        &["../../protos/"],
    )?;
    Ok(())
}
