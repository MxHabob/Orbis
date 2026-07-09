//! Plugin Architecture — extensible platform for school-specific customizations.
//!
//! Every school is different. The plugin system allows extending Orbis without
//! modifying core code:
//!   • Noor integration (Saudi Arabia)
//!   • Custom behaviour point systems
//!   • Custom report formats
//!   • Custom admission rules
//!   • LMS integrations
//!
//! Plugins subscribe to domain events and can:
//!   - React to events (e.g. sync student to Noor on enrollment.created)
//!   - Register new AI actions (discoverable by AI agents)
//!   - Initialize state at startup

use std::sync::Arc;
use sea_orm::DatabaseConnection;
use tracing::{debug, warn};

use crate::errors::AppError;
use crate::event_bus::EventEnvelope;

/// An Orbis plugin — extends the platform with custom behaviour.
///
/// # Example
///
/// ```rust,ignore
/// struct NoorIntegrationPlugin;
///
/// impl OrbisPlugin for NoorIntegrationPlugin {
///     fn name(&self) -> &str { "noor-integration" }
///     fn version(&self) -> &str { "1.0.0" }
///     fn subscribed_events(&self) -> &[&str] {
///         &["enrollment.created", "grade.submitted"]
///     }
///     async fn on_event(&self, db: &DatabaseConnection, org_id: &str,
///                        event: &EventEnvelope) -> Result<(), AppError> {
///         // Sync to Noor...
///         Ok(())
///     }
/// }
/// ```
#[allow(async_fn_in_trait)]
pub trait OrbisPlugin: Send + Sync {
    /// Unique plugin name.
    fn name(&self) -> &str;

    /// Plugin version (semver).
    fn version(&self) -> &str;

    /// Event types this plugin subscribes to.
    fn subscribed_events(&self) -> &[&str];

    /// Called when a subscribed event occurs.
    async fn on_event(
        &self,
        db: &DatabaseConnection,
        org_id: &str,
        event: &EventEnvelope,
    ) -> Result<(), AppError>;

    /// Called once at application startup. Override to initialize state.
    async fn initialize(&self, _db: &DatabaseConnection) -> Result<(), AppError> {
        Ok(())
    }
}

/// Registry of all loaded plugins.
pub struct PluginRegistry {
    plugins: Vec<Arc<dyn OrbisPlugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    /// Register a plugin.
    pub fn register(&mut self, plugin: Arc<dyn OrbisPlugin>) {
        debug!(
            plugin = plugin.name(),
            version = plugin.version(),
            events = ?plugin.subscribed_events(),
            "Registered plugin"
        );
        self.plugins.push(plugin);
    }

    /// Initialize all registered plugins.
    pub async fn initialize_all(&self, db: &DatabaseConnection) -> Result<(), AppError> {
        for plugin in &self.plugins {
            debug!(plugin = plugin.name(), "Initializing plugin");
            plugin.initialize(db).await.map_err(|e| {
                AppError::internal(format!(
                    "Plugin '{}' initialization failed: {}",
                    plugin.name(),
                    e
                ))
            })?;
        }
        Ok(())
    }

    /// Dispatch an event to all subscribed plugins.
    ///
    /// Plugin errors are logged but do not interrupt the main operation.
    pub async fn dispatch(
        &self,
        db: &DatabaseConnection,
        org_id: &str,
        event: &EventEnvelope,
    ) {
        for plugin in &self.plugins {
            if plugin
                .subscribed_events()
                .contains(&event.event_type.as_str())
            {
                debug!(
                    plugin = plugin.name(),
                    event = %event.event_type,
                    "Dispatching event to plugin"
                );
                if let Err(e) = plugin.on_event(db, org_id, event).await {
                    warn!(
                        plugin = plugin.name(),
                        event = %event.event_type,
                        error = %e,
                        "Plugin error — continuing"
                    );
                }
            }
        }
    }

    /// Number of registered plugins.
    pub fn count(&self) -> usize {
        self.plugins.len()
    }

    /// Get all plugin names and versions.
    pub fn list(&self) -> Vec<(&str, &str)> {
        self.plugins
            .iter()
            .map(|p| (p.name(), p.version()))
            .collect()
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}
