use orbis_server::ApiDoc;
use utoipa::OpenApi;
use std::fs::File;
use std::io::Write;

fn main() {
    let doc = ApiDoc::openapi();
    let json = doc.to_json().expect("Failed to serialize OpenAPI document");

    let path = "openapi.json";
    let mut file = File::create(path).expect("Failed to create openapi.json file");
    file.write_all(json.as_bytes()).expect("Failed to write openapi.json");

    println!("OpenAPI documentation exported to {}", path);
}
