use axum::serve;
use mdns_sd::{ServiceDaemon, ServiceInfo};
use orbis_server::app_router;
use orbis_server::state::AppState;
use sea_orm::{ConnectOptions, Database};
use std::net::IpAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

fn validate_jwt_secret(secret: Option<String>) -> Result<String, &'static str> {
    let secret = secret
        .map(|value| value.trim().to_owned())
        .filter(|value| value.len() >= 32)
        .filter(|value| {
            let normalized = value.to_ascii_lowercase();
            !normalized.contains("replace-with")
                && !normalized.contains("random-32-plus")
                && normalized != "orbis_dev_super_secret_jwt_key_2026_educational_platform"
                && normalized != "change-me"
                && normalized != "changeme"
        })
        .ok_or("JWT_SECRET must be a non-placeholder value of at least 32 characters")?;
    Ok(secret)
}

fn start_lan_discovery(
    bind_host: &str,
    port: u16,
    organization: Option<&orbis_domain::entities::organization::Organization>,
) -> Option<ServiceDaemon> {
    if bind_host.eq_ignore_ascii_case("localhost")
        || bind_host
            .parse::<IpAddr>()
            .map(|address| address.is_loopback())
            .unwrap_or(false)
    {
        return None;
    }

    let machine_name = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .unwrap_or_else(|_| "server".to_string());
    let machine_label: String = machine_name
        .chars()
        .filter(|character| character.is_ascii_alphanumeric() || *character == '-')
        .take(40)
        .collect();
    let machine_label = if machine_label.is_empty() {
        "server"
    } else {
        &machine_label
    };
    let service_name = format!("Orbis-{machine_label}");
    let host_name = format!("orbis-{machine_label}.local.");

    let mut properties = vec![("app", "orbis".to_string())];
    if let Some(organization) = organization {
        properties.push(("organization_id", organization.id.to_string()));
        properties.push(("name", organization.name.clone()));
        if let Some(name_ar) = &organization.name_ar {
            properties.push(("name_ar", name_ar.clone()));
        }
    }

    let service = match ServiceInfo::new(
        "_orbis-api._tcp.local.",
        &service_name,
        &host_name,
        "",
        port,
        &properties[..],
    ) {
        Ok(service) => service.enable_addr_auto(),
        Err(error) => {
            tracing::warn!(%error, "Could not prepare Orbis LAN discovery.");
            return None;
        }
    };

    let daemon = match ServiceDaemon::new() {
        Ok(daemon) => daemon,
        Err(error) => {
            tracing::warn!(%error, "Could not start Orbis LAN discovery.");
            return None;
        }
    };
    if let Err(error) = daemon.register(service) {
        tracing::warn!(%error, "Could not advertise the Orbis API on the local network.");
        let _ = daemon.shutdown();
        return None;
    }

    tracing::info!(
        service = %service_name,
        port,
        "Advertising Orbis API for local network discovery."
    );
    Some(daemon)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args_os().skip(1).collect::<Vec<_>>();
    let migrate = match args.as_slice() {
        [] => false,
        [command] if command == "migrate" => true,
        _ => return Err(std::io::Error::other("Usage: orbis-server [migrate]").into()),
    };

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .init();

    // The API server owns its database configuration. Never discover a Desktop
    // user's local database or log credentials embedded in a database URL.
    let db_url = orbis_infra::config::resolve_database_url()?;
    tracing::info!("Connecting to configured server database");

    // Connect to database
    let max_connections = match std::env::var("ORBIS_DB_MAX_CONNECTIONS") {
        Ok(value) => value
            .parse::<u32>()
            .ok()
            .filter(|count| *count > 0)
            .ok_or("ORBIS_DB_MAX_CONNECTIONS must be a positive integer")?,
        Err(std::env::VarError::NotPresent) => 10,
        Err(error) => return Err(error.into()),
    };
    let mut database_options = ConnectOptions::new(db_url);
    database_options
        .max_connections(max_connections)
        .connect_timeout(std::time::Duration::from_secs(5))
        .acquire_timeout(std::time::Duration::from_secs(3))
        .sqlx_logging(false);
    let db = Database::connect(database_options).await?;

    use orbis_infra::migration::Migrator;
    use sea_orm_migration::{MigratorTrait, SchemaManager};
    if migrate {
        Migrator::up(&db, None).await?;
        tracing::info!("Orbis database schema is up to date");
        return Ok(());
    }

    let schema_manager = SchemaManager::new(&db);
    if !schema_manager.has_table("seaql_migrations").await? {
        return Err(std::io::Error::other(
            "Orbis database schema is not initialized. Run orbis-server migrate before starting the API server.",
        )
        .into());
    }
    let pending = Migrator::get_pending_migrations(&db).await?;
    if !pending.is_empty() {
        return Err(std::io::Error::other(format!(
            "The Orbis database has {} pending migration(s). Run orbis-server migrate before starting the API server.",
            pending.len()
        ))
        .into());
    }

    let server_secrets = orbis_infra::config::resolve_server_secrets()?;
    let jwt_secret = validate_jwt_secret(Some(server_secrets.jwt_secret))?;
    let mfa_encryption_key = server_secrets.mfa_encryption_key;

    let license_data_dir = std::env::var_os("ORBIS_LICENSE_DATA_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or(std::env::current_dir()?.join(".orbis-license"));
    let app_version = env!("CARGO_PKG_VERSION").to_string();
    let mulverse_url = std::env::var("MULVERSE_API_URL").ok();
    let mulverse_client = Arc::new(match mulverse_url.as_deref() {
        Some(url) => orbis_infra::adapters::mulverse_client::MulverseClient::with_base_url(url)
            .map_err(std::io::Error::other)?,
        None => orbis_infra::adapters::mulverse_client::MulverseClient::new()
            .map_err(std::io::Error::other)?,
    });
    let license_runtime: Arc<dyn orbis_app::ports::license::LicenseRuntime> = Arc::new(
        orbis_infra::adapters::license_runtime::MulverseLicenseRuntime::new(
            &license_data_dir,
            &app_version,
            mulverse_url.as_deref(),
        )?,
    );
    license_runtime.initialize().await?;
    let _license_sync_handle =
        orbis_infra::adapters::license_runtime::spawn_background_sync(license_runtime.clone(), 30);

    let state = AppState::build(
        db.clone(),
        license_runtime.clone(),
        mulverse_client.clone(),
        jwt_secret,
        mfa_encryption_key,
    );
    let autopilot_service = state.autopilot_service.clone();
    let org_repo = state.org_repo.clone();
    let report_service = state.report_service.clone();

    // Spawn Autopilot Background Scheduler (checks every 60 seconds)
    let autopilot_scheduler = Arc::new(orbis_infra::autopilot::scheduler::AutopilotScheduler::new(
        autopilot_service.clone(),
        org_repo.clone(),
    ));
    let _scheduler_handle = autopilot_scheduler.spawn(60);

    let report_cleanup_service = report_service.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60 * 60));
        loop {
            interval.tick().await;
            match report_cleanup_service.cleanup_expired_reports().await {
                Ok(removed) if removed > 0 => {
                    tracing::info!(removed, "Expired report files cleaned up");
                }
                Ok(_) => {}
                Err(error) => {
                    tracing::error!(error = %error, "Expired report cleanup failed");
                }
            }
        }
    });

    // Spawn Notification cleanup background task (every 6 hours)
    let notification_cleanup = state.notification_service.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(6 * 60 * 60));
        loop {
            interval.tick().await;
            match notification_cleanup.cleanup_expired().await {
                Ok(removed) if removed > 0 => {
                    tracing::info!(removed, "Expired notifications cleaned up");
                }
                Ok(_) => {}
                Err(error) => {
                    tracing::error!(error = %error, "Expired notification cleanup failed");
                }
            }
        }
    });
    let organization = match state.org_repo.find_first().await {
        Ok(organization) => organization,
        Err(error) => {
            tracing::warn!(%error, "Could not load institution details for LAN discovery.");
            None
        }
    };

    // Start Server
    let bind_host = std::env::var("BIND_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let port_number = port.parse::<u16>()?;
    let addr = format!("{}:{}", bind_host, port);
    let listener = TcpListener::bind(&addr).await?;
    println!("Server running on http://{}", addr);

    // Build Axum Router
    let app = app_router(state);
    let mdns_daemon = start_lan_discovery(&bind_host, port_number, organization.as_ref());

    let serve_result = serve(listener, app).await;
    if let Some(daemon) = mdns_daemon {
        if let Err(error) = daemon.shutdown() {
            tracing::warn!(%error, "Could not stop the Orbis LAN discovery service cleanly.");
        }
    }
    serve_result?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_jwt_secret;

    #[test]
    fn jwt_secret_validation_rejects_published_example_value() {
        assert!(
            validate_jwt_secret(Some(
                "replace-with-a-random-32-plus-character-secret".to_string()
            ))
            .is_err()
        );
    }

    #[test]
    fn jwt_secret_validation_accepts_configured_secret() {
        assert_eq!(
            validate_jwt_secret(Some(
                "a-private-random-secret-with-enough-length".to_string()
            ))
            .unwrap(),
            "a-private-random-secret-with-enough-length"
        );
    }
}
