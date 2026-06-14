// Infra database crate entrypoint
// Currently, core-domain directly includes DB models and repo queries.
// This crate acts as an integration and migration entry point.
pub fn info() -> &'static str {
    "infra-database"
}
