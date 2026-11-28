use anyhow::{Result, anyhow};
use clap::Subcommand;
use std::path::PathBuf;

use crate::core::config::CliState;
use crate::core::license;
use crate::ui::{output, prompts};

#[derive(Subcommand, Clone)]
pub enum LicenseCommand {
    /// Show current license status
    Status,
    /// Import a license_snapshot.json file into the server
    Import { path: PathBuf },
    /// Activate a product key
    Activate {
        key: String,
        #[arg(long)]
        org_id: Option<String>,
    },
    /// Refresh license with Mulverse Platform
    Refresh {
        #[arg(long)]
        org_id: Option<String>,
    },
    /// Deactivate the current installation
    Deactivate,
}

pub async fn execute(sub: Option<LicenseCommand>) -> Result<()> {
    let state = CliState::load()
        .ok_or_else(|| anyhow!("Orbis is not configured yet. Run `orbis init` first."))?;

    match sub {
        Some(LicenseCommand::Status) => show_status(&state).await,
        Some(LicenseCommand::Import { path }) => {
            let dto = license::import_snapshot_file(&state, &path).await?;
            print_license_summary(&dto);
            Ok(())
        }
        Some(LicenseCommand::Activate { key, org_id }) => {
            let id = org_id
                .as_deref()
                .and_then(|s| uuid::Uuid::parse_str(s).ok())
                .unwrap_or_else(uuid::Uuid::nil);
            let dto = license::activate_key(&state, &key, id, None).await?;
            print_license_summary(&dto);
            Ok(())
        }
        Some(LicenseCommand::Refresh { org_id }) => {
            let id = org_id
                .as_deref()
                .and_then(|s| uuid::Uuid::parse_str(s).ok());
            let dto = license::refresh(&state, id).await?;
            print_license_summary(&dto);
            Ok(())
        }
        Some(LicenseCommand::Deactivate) => {
            let dto = license::deactivate(&state).await?;
            print_license_summary(&dto);
            Ok(())
        }
        None => interactive_menu(&state).await,
    }
}

async fn show_status(state: &CliState) -> Result<()> {
    let dto = license::get_status(state).await?;
    print_license_summary(&dto);
    Ok(())
}

async fn interactive_menu(state: &CliState) -> Result<()> {
    show_status(state).await?;

    let options = vec![
        "1. Import license_snapshot.json",
        "2. Activate Product Key",
        "3. Refresh License",
        "4. Deactivate License",
        "5. Back",
    ];
    let choice = prompts::prompt_select("License action:", options)?;

    if choice.starts_with("1.") {
        let p = prompts::prompt_text("Path to license_snapshot.json:")?;
        let dto = license::import_snapshot_file(state, &PathBuf::from(p.trim().trim_matches('"')))
            .await?;
        print_license_summary(&dto);
    } else if choice.starts_with("2.") {
        let key = prompts::prompt_text("Product Key:")?;
        let org_str = prompts::prompt_text_with_default(
            "Organization UUID:",
            &uuid::Uuid::nil().to_string(),
        )?;
        let org_id = uuid::Uuid::parse_str(org_str.trim()).unwrap_or_else(|_| uuid::Uuid::nil());
        let dto = license::activate_key(state, key.trim(), org_id, None).await?;
        print_license_summary(&dto);
    } else if choice.starts_with("3.") {
        let dto = license::refresh(state, None).await?;
        print_license_summary(&dto);
    } else if choice.starts_with("4.") {
        if prompts::prompt_confirm("Deactivate current license?", false)? {
            let dto = license::deactivate(state).await?;
            print_license_summary(&dto);
        }
    }
    Ok(())
}

fn print_license_summary(dto: &orbis_domain::entities::license::LicenseStateDto) {
    let state_str = format!("{:?}", dto.state);
    let plan_str = dto.plan_name.as_deref().unwrap_or("None");
    let org_str = dto
        .organization_id
        .clone()
        .unwrap_or_else(|| "None".to_string());
    let valid_str = dto
        .valid_until
        .map(|d| d.to_rfc3339())
        .unwrap_or_else(|| "N/A".to_string());
    let operational_str = if dto.is_operational { "Yes" } else { "No" };

    output::print_summary_box(
        "ORBIS LICENSE STATUS",
        &[
            ("State", &state_str),
            ("Plan", plan_str),
            ("Operational", operational_str),
            ("Installation ID", &dto.installation_id),
            ("Organization ID", &org_str),
            ("Valid Until", &valid_str),
        ],
    );
}
