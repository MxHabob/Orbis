use anyhow::Result;
use crate::ui::output;

pub async fn execute() -> Result<()> {
    output::print_step("Managing Orbis License...");
    output::print_info("(Not yet fully implemented)");
    Ok(())
}
