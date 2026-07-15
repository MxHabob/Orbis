use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{info, error, warn};

/// A generic background job payload
pub trait Job: Send + Sync + 'static {
    fn execute(&self) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), String>> + Send>>;
    fn name(&self) -> &str;
}

#[derive(Clone)]
pub struct JobQueue {
    sender: mpsc::Sender<Box<dyn Job>>,
}

impl JobQueue {
    /// Create a new Job Queue and spawn the background worker loop
    pub fn new(capacity: usize) -> Self {
        let (sender, mut receiver) = mpsc::channel::<Box<dyn Job>>(capacity);

        tokio::spawn(async move {
            info!("Background Job Worker started (capacity: {})", capacity);
            while let Some(job) = receiver.recv().await {
                let name = job.name().to_string();
                info!("Executing background job: {}", name);
                
                match job.execute().await {
                    Ok(_) => info!("Job completed successfully: {}", name),
                    Err(e) => error!("Job failed: {} - Error: {}", name, e),
                }
            }
            warn!("Background Job Worker shut down");
        });

        Self { sender }
    }

    /// Enqueue a job. Returns an error if the queue is full.
    pub async fn enqueue<J: Job>(&self, job: J) -> Result<(), String> {
        let name = job.name().to_string();
        self.sender.send(Box::new(job)).await.map_err(|_| {
            let msg = format!("Job queue is closed, cannot enqueue: {}", name);
            error!("{}", msg);
            msg
        })
    }
    
    /// Try to enqueue without blocking. Returns an error if full.
    pub fn try_enqueue<J: Job>(&self, job: J) -> Result<(), String> {
        let name = job.name().to_string();
        self.sender.try_send(Box::new(job)).map_err(|e| {
            let msg = format!("Failed to enqueue job: {} - {}", name, e);
            warn!("{}", msg);
            msg
        })
    }
}
