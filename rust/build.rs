use std::io::Result;

fn main() -> Result<()> {
    let mut config = prost_build::Config::new();
    config.type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]");
    config.type_attribute(".", "#[serde(rename_all=\"camelCase\")]");
    config.protoc_arg("-I=../proto");
    config.compile_protos(&["../proto/scene.proto"], &["src/"])?;
    Ok(())
}