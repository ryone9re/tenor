use super::{Labels, Timestamp};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Container identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContainerId(pub String);

impl fmt::Display for ContainerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ContainerId {
    fn from(s: String) -> Self {
        ContainerId(s)
    }
}

impl AsRef<str> for ContainerId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Container state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContainerState {
    Running,
    Exited,
    Paused,
    Restarting,
    Dead,
    Unknown,
}

impl fmt::Display for ContainerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Running => write!(f, "Running"),
            Self::Exited => write!(f, "Exited"),
            Self::Paused => write!(f, "Paused"),
            Self::Restarting => write!(f, "Restarting"),
            Self::Dead => write!(f, "Dead"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Port mapping
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortMapping {
    pub container_port: u16,
    pub host_port: Option<u16>,
    pub host_ip: Option<String>,
    pub protocol: PortProtocol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PortProtocol {
    Tcp,
    Udp,
}

impl fmt::Display for PortProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Tcp => write!(f, "tcp"),
            Self::Udp => write!(f, "udp"),
        }
    }
}

/// Container summary (for list views)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
    pub id: ContainerId,
    pub name: String,
    pub image: String,
    pub state: ContainerState,
    pub status: String,
    pub created_at: Timestamp,
    pub labels: Labels,
    pub ports: Vec<PortMapping>,
}

/// Mount information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mount {
    pub source: String,
    pub destination: String,
    pub mode: String,
    pub rw: bool,
}

/// Network settings summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSettings {
    pub networks: Vec<String>,
    pub ip_address: Option<String>,
}

/// Detailed container information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerDetail {
    pub id: ContainerId,
    pub name: String,
    pub image: String,
    pub state: ContainerState,
    pub status: String,
    pub created_at: Timestamp,
    pub labels: Labels,
    pub ports: Vec<PortMapping>,
    pub command: Vec<String>,
    pub entrypoint: Vec<String>,
    pub env: Vec<String>,
    pub mounts: Vec<Mount>,
    pub network_settings: NetworkSettings,
}

/// Container filter options
#[derive(Debug, Clone, Default)]
pub struct ContainerFilter {
    pub name: Option<String>,
    pub state: Option<ContainerState>,
    pub labels: Vec<(String, String)>,
}

/// Container delete options
#[derive(Debug, Clone, Default)]
pub struct DeleteContainerOpts {
    pub force: bool,
    pub remove_volumes: bool,
}
