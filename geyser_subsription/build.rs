fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .type_attribute("shyft.geyser.Block.transaction", "#[allow(dead_code)]")
        .type_attribute("shyft.geyser.Block.transaction_status_meta", "#[allow(dead_code)]")
        .type_attribute("shyft.geyser.Block.error", "#[allow(dead_code)]")
        .type_attribute("shyft.geyser.Block.rewards", "#[allow(dead_code)]")
        .type_attribute("shyft.geyser.Block.unix_timestamp", "#[allow(dead_code)]")
        .type_attribute("shyft.geyser.Block.block_height", "#[allow(dead_code)]")
        .compile_protos(&["proto/shyft.geyser.proto"], &["proto"])?;
    Ok(())
}