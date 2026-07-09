//! Finance write commands.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordPaymentCmd {
    pub org_id: String,
    pub enrollment_id: String,
    pub student_fee_id: String,
    pub amount: f64,
    pub method: String,
    pub treasury_id: String,
    pub reference: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFeeTemplateCmd {
    pub org_id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub category: String,
    pub default_amount: f64,
    pub is_mandatory: bool,
    pub frequency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignFeeCmd {
    pub org_id: String,
    pub enrollment_id: String,
    pub fee_template_id: String,
    pub amount_override: Option<f64>,
}
