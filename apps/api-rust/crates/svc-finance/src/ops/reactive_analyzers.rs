use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::reactive::{DetectedInsight, EventAnalyzer};

/// Analyzes financial events to generate insights.
pub struct FinanceAnalyzer;

impl FinanceAnalyzer {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EventAnalyzer for FinanceAnalyzer {
    fn name(&self) -> &str {
        "FinanceAnalyzer"
    }

    fn interested_in(&self) -> &[&str] {
        &[
            "finance.payment.received",
            "finance.payment.failed",
            "finance.invoice.overdue",
        ]
    }

    async fn analyze(
        &self,
        _db: &DatabaseConnection,
        _org_id: &str,
        event: &EventEnvelope,
    ) -> Vec<DetectedInsight> {
        let mut insights = Vec::new();

        if event.event_type == "finance.invoice.overdue" {
            insights.push(DetectedInsight {
                insight_type: "invoice_overdue_alert".to_string(),
                entity_type: Some("invoice".to_string()),
                entity_id: Some(event.entity_id.clone()),
                title: "Overdue Invoice Alert".to_string(),
                title_ar: Some("تنبيه فاتورة متأخرة".to_string()),
                summary: "An invoice has become overdue and requires follow-up.".to_string(),
                summary_ar: Some("أصبحت الفاتورة متأخرة وتحتاج إلى متابعة.".to_string()),
                severity: "warning".to_string(),
                confidence: 1.0,
                impact_score: Some(0.6),
                suggested_action: Some("follow_up_invoice".to_string()),
                suggested_params: None,
            });
        }

        insights
    }
}
