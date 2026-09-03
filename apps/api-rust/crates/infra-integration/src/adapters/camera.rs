//! Camera Integration Adapter — skeleton for IP camera / attendance systems.
//!
//! Supports:
//!   - Hikvision IVMS
//!   - Dahua
//!   - Generic ONVIF
//!
//! Events produced:
//!   - `camera.face_detected` — face recognized at entrance
//!   - `camera.motion_detected` — motion in restricted area
//!   - `camera.attendance_marked` — auto-attendance from face recognition

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::traits::{IntegrationProvider, IntegrationEvent, AdapterStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraConfig {
    pub camera_id: String,
    pub name: String,
    pub ip_address: String,
    pub port: u16,
    pub protocol: String, // "onvif" | "hikvision" | "dahua"
    pub username: Option<String>,
    pub password: Option<String>,
    pub location: Option<String>,
}

pub struct CameraAdapter {
    config: CameraConfig,
    status: AdapterStatus,
}

impl CameraAdapter {
    pub fn new(config: CameraConfig) -> Self {
        Self {
            config,
            status: AdapterStatus::Disconnected,
        }
    }
}

#[async_trait]
impl IntegrationProvider for CameraAdapter {
    fn adapter_name(&self) -> &str {
        &self.config.name
    }

    fn category(&self) -> &str {
        "camera"
    }

    fn status(&self) -> AdapterStatus {
        self.status.clone()
    }

    async fn connect(&mut self) -> Result<(), String> {
        // TODO: Implement actual camera connection
        // For now, mark as connected (skeleton)
        tracing::info!(
            camera = %self.config.name,
            ip = %self.config.ip_address,
            "Camera adapter connected (skeleton)"
        );
        self.status = AdapterStatus::Connected;
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), String> {
        self.status = AdapterStatus::Disconnected;
        Ok(())
    }

    async fn poll_events(&self) -> Result<Vec<IntegrationEvent>, String> {
        // TODO: Poll camera for face detection / attendance events
        Ok(vec![])
    }
}
