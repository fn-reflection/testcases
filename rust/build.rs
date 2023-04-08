fn main() -> std::io::Result<()> {
    let config: prost_build_config::BuildConfig =
        serde_yaml::from_str(include_str!("build_config.yml")).unwrap();
    prost_build_config::Builder::from(config).build_protos();
    Ok(())
}
