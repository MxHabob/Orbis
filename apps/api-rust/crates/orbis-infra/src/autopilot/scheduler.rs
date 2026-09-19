use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{info, warn, error};
use uuid::Uuid;

use orbis_app::context::RequestContext;
use orbis_app::ports::repositories::OrganizationRepository;
use orbis_app::services::AutopilotService;
use orbis_domain::iam::actor::Actor;

pub struct AutopilotScheduler {
    service: Arc<AutopilotService>,
    org_repo: Arc<dyn OrganizationRepository>,
}

impl AutopilotScheduler {
    pub fn new(service: Arc<AutopilotService>, org_repo: Arc<dyn OrganizationRepository>) -> Self {
        Self { service, org_repo }
    }

    /// Spawns the background scheduler tokio task loop.
    pub fn spawn(self: Arc<Self>, interval_secs: u64) -> tokio::task::JoinHandle<()> {
        info!("Spawning Orbis Autopilot background scheduler (interval: {}s)...", interval_secs);
        tokio::spawn(async move {
            self.run_loop(interval_secs).await;
        })
    }

    async fn run_loop(&self, interval_secs: u64) {
        info!("Orbis Autopilot Scheduler worker loop started");
        loop {
            match self.org_repo.list_all().await {
                Ok(orgs) => {
                    for org in orgs {
                        let ctx = RequestContext {
                            tenant_id: org.id,
                            actor: Actor::Autopilot,
                            request_id: Uuid::new_v4().to_string(),
                        };

                        if let Err(e) = self.service.run_tick(&ctx, org.id, false).await {
                            warn!("Autopilot evaluation tick failed for org {} ({}): {:?}", org.name, org.id, e);
                        }
                    }
                }
                Err(e) => {
                    error!("Autopilot Scheduler error querying organizations: {:?}", e);
                }
            }

            sleep(Duration::from_secs(interval_secs)).await;
        }
    }
}
