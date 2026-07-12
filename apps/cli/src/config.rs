use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CliConfig {
    pub db_url: String,
    pub install_path: String,
}

impl CliConfig {
    pub fn default() -> Self {
        let identifier = if cfg!(debug_assertions) {
            "com.mulverse.orbis.dev"
        } else {
            "com.mulverse.orbis.svelte"
        };

        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            if let Some(mut data_dir) = dirs::data_dir() {
                data_dir.push(identifier);
                data_dir.push("orbis.db");
                format!("sqlite://{}?mode=rwc", data_dir.to_string_lossy())
            } else {
                "sqlite://orbis.db?mode=rwc".to_string()
            }
        });

        Self {
            db_url,
            install_path: "./".to_string(),
        }
    }

    pub fn get_app_executable_paths() -> Vec<std::path::PathBuf> {
        let app_name = if cfg!(windows) { "Orbis-Svelte.exe" } else { "Orbis-Svelte" };
        let product_name = if cfg!(windows) { "Orbis.exe" } else { "Orbis" };
        
        let mut paths = vec![
            std::path::Path::new(app_name).to_path_buf(),
            std::path::Path::new(product_name).to_path_buf(),
        ];

        // Add common Windows installation paths
        if cfg!(windows) {
            if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
                paths.push(std::path::Path::new(&local_app_data).join("Programs").join("Orbis").join(product_name));
                paths.push(std::path::Path::new(&local_app_data).join("Programs").join("Orbis").join(app_name));
                paths.push(std::path::Path::new(&local_app_data).join("Orbis").join(product_name));
                paths.push(std::path::Path::new(&local_app_data).join("com.mulverse.orbis.svelte").join(product_name));
            }
            if let Ok(program_files) = std::env::var("ProgramFiles") {
                paths.push(std::path::Path::new(&program_files).join("Orbis").join(product_name));
                paths.push(std::path::Path::new(&program_files).join("Orbis").join(app_name));
            }
            if let Ok(program_files_x86) = std::env::var("ProgramFiles(x86)") {
                paths.push(std::path::Path::new(&program_files_x86).join("Orbis").join(product_name));
            }
        }

        // Built paths (fallback priority)
        paths.extend(vec![
            std::path::Path::new("apps/desktop-svelte/src-tauri/target/release").join(app_name),
            std::path::Path::new("apps/desktop-svelte/src-tauri/target/debug").join(app_name),
            std::path::Path::new("../desktop-svelte/src-tauri/target/release").join(app_name),
            std::path::Path::new("../desktop-svelte/src-tauri/target/debug").join(app_name),
        ]);

        paths
    }
}
