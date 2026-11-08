fn main() {
    println!("cargo:rerun-if-env-changed=ORBIS_SOLVER_MANIFEST_URL");
    println!("cargo:rerun-if-env-changed=ORBIS_SOLVER_PUBLIC_KEY_B64");
    if let Ok(url) = std::env::var("ORBIS_SOLVER_MANIFEST_URL") {
        println!("cargo:rustc-env=ORBIS_SOLVER_MANIFEST_URL={url}");
    }
    if let Ok(key) = std::env::var("ORBIS_SOLVER_PUBLIC_KEY_B64") {
        println!("cargo:rustc-env=ORBIS_SOLVER_PUBLIC_KEY_B64={key}");
    }

    tauri_build::build()
}
