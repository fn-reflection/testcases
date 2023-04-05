fn main() -> std::io::Result<()> {
    prost_build::compile_protos(
        &[
            "protos/board/v1/board.proto",
            "protos/test_proto/v1/test_proto.proto",
        ],
        &["protos/"],
    )?;
    Ok(())
}
