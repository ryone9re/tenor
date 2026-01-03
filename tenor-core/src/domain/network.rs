use super::Labels;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Network identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NetworkId(pub String);

impl fmt::Display for NetworkId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for NetworkId {
    fn from(s: String) -> Self {
        NetworkId(s)
    }
}

impl AsRef<str> for NetworkId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Network summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub id: NetworkId,
    pub name: String,
    pub driver: String,
    pub scope: String,
    pub internal: bool,
    pub labels: Labels,
}

/// Detailed network information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDetail {
    pub id: NetworkId,
    pub name: String,
    pub driver: String,
    pub scope: String,
    pub internal: bool,
    pub labels: Labels,
    pub ipam: Option<IpamConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamConfig {
    pub driver: String,
    pub config: Vec<IpamSubnet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpamSubnet {
    pub subnet: String,
    pub gateway: Option<String>,
}

/// Network filter options
#[derive(Debug, Clone, Default)]
pub struct NetworkFilter {
    pub name: Option<String>,
}
