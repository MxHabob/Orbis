use std::{fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../openapi/orbis-openapi.json");
    let parent = output
        .parent()
        .ok_or("OpenAPI output directory is missing")?;
    fs::create_dir_all(parent)?;
    fs::write(output, api_gateway::openapi::json())?;
    Ok(())
}