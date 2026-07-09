//! Reactive Pipeline — real-time event analysis after every domain event.
//!
//! When an event is published (e.g. `enrollment.created`), the pipeline
//! runs registered `EventAnalyzer`s that produce `DetectedInsight`s:
//!
//!   enrollment.created → ClassBalanceAnalyzer   → "Class 3A is at 35/35 capacity"
//!                      → SiblingDetector        → "Family has 3 students — suggest sibling discount"
//!   grade.submitted    → AtRiskPredictor        → "Student grade dropped 20% — at risk of failing"
//!                      → GradeAnomalyDetector   → "15 identical grades — suspicious"
//!   payment.overdue    → OverdueAnalyzer        → "5 students past grace period"
//!
//! This transforms InsightEngine from batch analysis to real-time reactive intelligence.

use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use serde::Serialize;
use tracing::debug;

use crate::event_bus::EventEnvelope;

/// A detected insight — produced by an EventAnalyzer.
#[derive(Debug, Clone, Serialize)]
pub struct DetectedInsight {
    /// Type of insight (e.g. "at_risk_student", "class_capacity", "grade_anomaly").
    pub insight_type: String,
    /// The entity this insight is about (if any).
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    /// Human-readable title.
    pub title: String,
    pub title_ar: Option<String>,
    /// Detailed summary.
    pub summary: String,
    pub summary_ar: Option<String>,
    /// Severity: "info", "warning", "critical".
    pub severity: String,
    /// Confidence score (0.0 — 1.0).
    pub confidence: f64,
    /// Impact score (optional, higher = more impactful).
    pub impact_score: Option<f64>,
    /// Suggested action code (from ActionRegistry).
    pub suggested_action: Option<String>,
    /// Parameters for the suggested action.
    pub suggested_params: Option<serde_json::Value>,
}

/// An event analyzer that reacts to domain events and produces insights.
///
/// Each analyzer declares which events it's interested in.
/// The pipeline only calls `analyze()` for matching events.
#[async_trait]
pub trait EventAnalyzer: Send + Sync {
    /// Human-readable name of this analyzer.
    fn name(&self) -> &str;

    /// Event types this analyzer is interested in.
    fn interested_in(&self) -> &[&str];

    /// Analyze an event and produce zero or more insights.
    async fn analyze(
        &self,
        db: &DatabaseConnection,
        org_id: &str,
        event: &EventEnvelope,
    ) -> Vec<DetectedInsight>;
}

/// The reactive pipeline — dispatches events to registered analyzers.
pub struct ReactivePipeline {
    analyzers: Vec<Arc<dyn EventAnalyzer>>,
}

impl ReactivePipeline {
    pub fn new() -> Self {
        Self {
            analyzers: Vec::new(),
        }
    }

    /// Register a new event analyzer.
    pub fn register(&mut self, analyzer: Arc<dyn EventAnalyzer>) {
        debug!(analyzer = analyzer.name(), "Registered reactive analyzer");
        self.analyzers.push(analyzer);
    }

    /// Process an event through all interested analyzers.
    ///
    /// Returns all detected insights. The caller is responsible for
    /// persisting insights (e.g. to `ai_insight` table).
    pub async fn process(
        &self,
        db: &DatabaseConnection,
        org_id: &str,
        event: &EventEnvelope,
    ) -> Vec<DetectedInsight> {
        let mut insights = Vec::new();

        for analyzer in &self.analyzers {
            if analyzer.interested_in().contains(&event.event_type.as_str()) {
                debug!(
                    analyzer = analyzer.name(),
                    event = %event.event_type,
                    "Running reactive analyzer"
                );
                let mut analyzer_insights = analyzer.analyze(db, org_id, event).await;
                if !analyzer_insights.is_empty() {
                    debug!(
                        analyzer = analyzer.name(),
                        count = analyzer_insights.len(),
                        "Analyzer produced insights"
                    );
                }
                insights.append(&mut analyzer_insights);
            }
        }

        insights
    }

    /// Number of registered analyzers.
    pub fn analyzer_count(&self) -> usize {
        self.analyzers.len()
    }
}

impl Default for ReactivePipeline {
    fn default() -> Self {
        Self::new()
    }
}
