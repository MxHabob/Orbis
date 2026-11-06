fn main() {
    println!("cargo:rerun-if-env-changed=ORBIS_SOLVER_SHA256");
    if let Ok(hash) = std::env::var("ORBIS_SOLVER_SHA256") {
        println!("cargo:rustc-env=ORBIS_SOLVER_SHA256={hash}");
    }

    tauri_build::build()
}
