use std::sync::Arc;
use tokio::time::{sleep, Duration};
use orbis_app::autopilot::engine::AutopilotEngine;
use crate::autopilot::task_repo::AutopilotTaskRepository;
use orbis_domain::iam::actor::Actor;
use orbis_app::context::RequestContext;
use uuid::Uuid;

pub struct AutopilotScheduler {
    engine: Arc<AutopilotEngine>,
    task_repo: Arc<dyn AutopilotTaskRepository>,
}

impl AutopilotScheduler {
    pub fn new(engine: Arc<AutopilotEngine>, task_repo: Arc<dyn AutopilotTaskRepository>) -> Self {
        Self { engine, task_repo }
    }

    /// Starts the background scheduler loop.
    pub async fn start(&self) {
        println!("🚀 Autopilot Scheduler started...");
        loop {
            match self.task_repo.find_pending_tasks().await {
                Ok(tasks) => {
                    for task in tasks {
                        // Create a System context for Autopilot execution
                        let ctx = RequestContext {
                            tenant_id: task.org_id,
                            actor: Actor::Autopilot,
                            request_id: Uuid::new_v4().to_string(),
                        };

                        if let Err(e) = self.engine.process_task(&ctx, task.clone()).await {
                            eprintln!("❌ Autopilot Task {} failed: {:?}", task.id, e);
                            // Here we would save task status as Failed.
                        }
                    }
                }
                Err(e) => eprintln!("Error fetching pending tasks for Autopilot: {:?}", e),
            }

            // Poll every 60 seconds (simulated)
            sleep(Duration::from_secs(60)).await;
        }
    }
}
