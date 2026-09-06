use orbis_server::ApiDoc;
use utoipa::OpenApi;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let doc = ApiDoc::openapi();
    let json = doc.to_json().expect("Failed to serialize OpenAPI document");

    let args: Vec<String> = std::env::args().collect();
    let paths: Vec<String> = if args.len() > 1 {
        vec![args[1].clone()]
    } else {
        vec![
            "apps/api-rust/openapi.json".to_string(),
            "openapi.json".to_string(),
        ]
    };

    for path_str in paths {
        let path = Path::new(&path_str);
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                let _ = std::fs::create_dir_all(parent);
            }
        }
        if let Ok(mut file) = File::create(path) {
            let _ = file.write_all(json.as_bytes());
            println!("OpenAPI documentation exported to {}", path_str);
        }
    }
}

