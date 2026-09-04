use shared_kernel::job_queue::Job;
use shared_kernel::context::ServiceContext;
use std::collections::HashMap;

pub struct SnapshotJob {
    pub ctx: ServiceContext,
    pub student_id: String,
    pub period_id: String,
    pub class_subject_id: String,
    pub policy_id: String,
    pub scale_id: String,
    pub raw_scores: HashMap<String, f64>,
}

impl Job for SnapshotJob {
    fn execute(&self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), String>> + Send>> {
        let ctx = self.ctx.clone();
        let student_id = self.student_id.clone();
        let period_id = self.period_id.clone();
        let class_subject_id = self.class_subject_id.clone();
        let policy_id = self.policy_id.clone();
        let scale_id = self.scale_id.clone();
        let raw_scores = self.raw_scores.clone();

        Box::pin(async move {
            // Call the actual synchronous logic wrapped in async
            crate::ops::snapshot_generator::execute_snapshot_generation(
                &ctx,
                &student_id,
                &period_id,
                &class_subject_id,
                &policy_id,
                &scale_id,
                raw_scores
            ).await.map_err(|e| e.to_string())?;
            Ok(())
        })
    }

    fn name(&self) -> &str {
        "SnapshotJob"
    }
}
