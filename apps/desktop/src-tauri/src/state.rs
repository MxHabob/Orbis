use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    database_target: Arc<str>,
    pub server_state: orbis_server::state::AppState,
    pub server_router: axum::Router,
    pub solver_sidecar: Arc<crate::features::solver_sidecar::SolverSidecarManager>,
    pub camera_stream_manager: Arc<crate::features::camera_stream::CameraStreamManager>,
    pub app_data_dir: std::path::PathBuf,
}

impl AppState {
    pub(crate) fn uses_database(&self, database_url: &str) -> bool {
        self.database_target.as_ref() == database_url
    }

    pub async fn init(
        app_data_dir: &std::path::Path,
        db_url: &str,
        migrate: bool,
    ) -> Result<Self, String> {
        let mut connect_options = ConnectOptions::new(db_url.to_owned());
        connect_options
            .max_connections(1)
            .connect_timeout(std::time::Duration::from_secs(5))
            .acquire_timeout(std::time::Duration::from_secs(3))
            .sqlx_logging(false);

        let db = Database::connect(connect_options)
            .await
            .map_err(|e| e.to_string())?;

        if db_url.starts_with("sqlite:") {
            Self::configure_sqlite_pragmas(&db).await?;
        }

        let mfa_encryption_key = Self::load_or_generate_mfa_encryption_key(app_data_dir)?;

        use orbis_infra::migration::Migrator;
        use sea_orm_migration::{MigratorTrait, SchemaManager};
        if migrate {
            Migrator::up(&db, None).await.map_err(|e| e.to_string())?;
        } else {
            if !SchemaManager::new(&db)
                .has_table("seaql_migrations")
                .await
                .map_err(|e| e.to_string())?
            {
                let pending_count = Migrator::migrations().len();
                return Err(format!(
                    "SCHEMA_MIGRATION_REQUIRED:{pending_count}:Orbis schema has not been initialized"
                ));
            }
            let pending = Migrator::get_pending_migrations(&db)
                .await
                .map_err(|e| e.to_string())?;
            if !pending.is_empty() {
                return Err(format!(
                    "SCHEMA_MIGRATION_REQUIRED:{}:Database schema updates are required",
                    pending.len()
                ));
            }
        }

        let app_version = env!("CARGO_PKG_VERSION").to_string();
        let mulverse_url = std::env::var("MULVERSE_API_URL").ok();
        let license_data_dir = std::env::var_os("ORBIS_LICENSE_DATA_DIR")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| app_data_dir.to_path_buf());
        let license_runtime: Arc<dyn orbis_app::ports::license::LicenseRuntime> = Arc::new(
            orbis_infra::adapters::license_runtime::MulverseLicenseRuntime::new(
                &license_data_dir,
                &app_version,
                mulverse_url.as_deref(),
            )
            .map_err(|e| format!("Failed to create license runtime: {e}"))?,
        );
        license_runtime
            .initialize()
            .await
            .map_err(|e| format!("Failed to initialize license runtime: {e}"))?;

        let jwt_secret = Self::load_or_generate_jwt_secret(app_data_dir)?;
        let server_mulverse = Arc::new(
            orbis_infra::adapters::mulverse_client::MulverseClient::new()
                .map_err(|e| e.to_string())?,
        );
        let server_state = orbis_server::state::AppState::build(
            db.clone(),
            license_runtime.clone(),
            server_mulverse,
            jwt_secret,
            mfa_encryption_key,
        );
        let server_router = orbis_server::app_router(server_state.clone());

        let solver_sidecar = Arc::new(crate::features::solver_sidecar::SolverSidecarManager::new(
            app_data_dir,
        )?);
        let camera_stream_manager = Arc::new(
            crate::features::camera_stream::CameraStreamManager::new(app_data_dir),
        );

        let autopilot_scheduler =
            Arc::new(orbis_infra::autopilot::scheduler::AutopilotScheduler::new(
                server_state.autopilot_service.clone(),
                server_state.org_repo.clone(),
            ));
        let _scheduler_handle = autopilot_scheduler.spawn(60);

        let _license_sync_handle =
            orbis_infra::adapters::license_runtime::spawn_background_sync(license_runtime, 30);

        Ok(Self {
            db,
            database_target: Arc::from(db_url),
            server_state,
            server_router,
            solver_sidecar,
            camera_stream_manager,
            app_data_dir: app_data_dir.to_path_buf(),
        })
    }

    async fn configure_sqlite_pragmas(db: &DatabaseConnection) -> Result<(), String> {
        let pragmas = [
            "PRAGMA journal_mode = WAL;",
            "PRAGMA synchronous = NORMAL;",
            "PRAGMA foreign_keys = ON;",
            "PRAGMA busy_timeout = 5000;",
            "PRAGMA temp_store = MEMORY;",
            "PRAGMA cache_size = -64000;",
            "PRAGMA mmap_size = 268435456;",
        ];
        for pragma in pragmas {
            db.execute_unprepared(pragma)
                .await
                .map_err(|e| format!("Failed to apply SQLite pragma '{pragma}': {e}"))?;
        }
        Ok(())
    }

    /// Load or generate a per-installation JWT secret.
    /// Stored in AppData/jwt_secret — never hardcoded.
    fn load_or_generate_jwt_secret(app_data_dir: &std::path::Path) -> Result<String, String> {
        let secret_path = app_data_dir.join("jwt_secret");
        if secret_path.exists() {
            std::fs::read_to_string(&secret_path)
                .map(|s| s.trim().to_string())
                .map_err(|e| format!("Failed to read JWT secret: {e}"))
        } else {
            use rand::Rng;
            let secret: String = rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(64)
                .map(char::from)
                .collect();
            if let Some(parent) = secret_path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create AppData dir: {e}"))?;
            }
            std::fs::write(&secret_path, &secret)
                .map_err(|e| format!("Failed to write JWT secret: {e}"))?;
            Ok(secret)
        }
    }

    fn load_or_generate_mfa_encryption_key(
        app_data_dir: &std::path::Path,
    ) -> Result<String, String> {
        let key_path = app_data_dir.join("mfa_encryption_key");
        if key_path.exists() {
            std::fs::read_to_string(&key_path)
                .map(|key| key.trim().to_string())
                .map_err(|error| format!("Failed to read MFA encryption key: {error}"))
        } else {
            use rand::Rng;
            let key: String = rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(64)
                .map(char::from)
                .collect();
            if let Some(parent) = key_path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|error| format!("Failed to create AppData dir: {error}"))?;
            }
            std::fs::write(&key_path, &key)
                .map_err(|error| format!("Failed to write MFA encryption key: {error}"))?;
            Ok(key)
        }
    }
}
