//! Official Calendar Templates
//!
//! Each template encapsulates the data from an officially published ministerial
//! or ministry-level decision that defines the academic calendar for a specific
//! country and academic year.

pub mod yemen_1447_1448;

pub use yemen_1447_1448::CalendarTemplateInfo;

/// Returns all available official calendar templates.
pub fn list_all_templates() -> Vec<CalendarTemplateInfo> {
    let mut templates = Vec::new();
    templates.extend(yemen_1447_1448::list_templates());
    templates
}

/// Look up a template by its code (e.g. `"YE-1447-1448"`).
pub fn find_template(code: &str) -> Option<CalendarTemplateInfo> {
    list_all_templates().into_iter().find(|t| t.code == code)
}
