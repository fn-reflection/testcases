fn main() -> std::io::Result<()> {
    prost_build::compile_protos(&["protos/board.proto"], &["protos/"])?;
    Ok(())
}
