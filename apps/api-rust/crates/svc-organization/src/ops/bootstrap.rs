use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
use serde::{Deserialize, Serialize};
use tracing::info;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;

use crate::entities::organization;
use crate::ops::org_unit;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlueprintPayload {
    pub org_name: String,
    pub org_name_ar: Option<String>,
    pub currency: String,
    pub org_type: String, // 'school', 'university', 'training_center'
    // This will be expanded later to include full blueprint config
}

/// The new Smart Organization Initialization Engine (Blueprint Composition).
/// This replaces `initialize_school_setup`.
pub async fn bootstrap_organization(
    ctx: &ServiceContext,
    payload: serde_json::Value,
) -> Result<(), AppError> {
    info!("Starting organization bootstrap via Blueprint Composition...");

    let blueprint: BlueprintPayload = serde_json::from_value(payload).unwrap_or_else(|_| BlueprintPayload {
        org_name: "New Organization".to_string(),
        org_name_ar: None,
        currency: "SAR".to_string(),
        org_type: "school".to_string(),
    });

    let now_rfc = chrono::Utc::now().to_rfc3339();

    // 1. Core Organization Config Update
    if let Some(org_model) = crate::entities::organization::Entity::find_by_id(ctx.org_id.clone()).one(&ctx.db).await? {
        let mut active_org: crate::entities::organization::ActiveModel = org_model.into();
        active_org.name = Set(blueprint.org_name.clone());
        if let Some(ar) = blueprint.org_name_ar {
            active_org.name_ar = Set(Some(ar));
        }
        active_org.currency = Set(blueprint.currency);
        active_org.updated_at = Set(now_rfc.clone());
        active_org.save(&ctx.db).await?;
    }

    // 2. Blueprint specific execution (To be expanded in Phase 2)
    // For now, we establish the root org unit to satisfy existing dependencies.
    let _root_unit = match org_unit::list(ctx).await?.into_iter().find(|u| u.unit_type == "root") {
        Some(u) => u,
        None => {
            org_unit::create(
                ctx,
                serde_json::json!({
                    "name": blueprint.org_name,
                    "unit_type": "root"
                }),
            )
            .await?
        }
    };

    info!("Bootstrap Phase 1 (Foundation) complete for org_id: {}", ctx.org_id);

    // TODO (Phase 2):
    // if blueprint.org_type == "university" { load_university_blueprint(...) } else { load_school_blueprint(...) }
    // which injects Academic Schemes, Grading Policies, Default Chart of Accounts, etc.

    Ok(())
}
