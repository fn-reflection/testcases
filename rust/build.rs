fn main() -> std::io::Result<()> {
    prost_build::compile_protos(&["src/board.proto"], &["src/"])?;
    Ok(())
}
