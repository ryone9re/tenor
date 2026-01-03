use super::Labels;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Volume name (volumes use names as identifiers)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VolumeName(pub String);

impl fmt::Display for VolumeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for VolumeName {
    fn from(s: String) -> Self {
        VolumeName(s)
    }
}

impl AsRef<str> for VolumeName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Volume summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Volume {
    pub name: VolumeName,
    pub driver: String,
    pub mountpoint: String,
    pub labels: Labels,
}

/// Detailed volume information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeDetail {
    pub name: VolumeName,
    pub driver: String,
    pub mountpoint: String,
    pub labels: Labels,
    pub scope: String,
}

/// Volume filter options
#[derive(Debug, Clone, Default)]
pub struct VolumeFilter {
    pub name: Option<String>,
    pub dangling: Option<bool>,
}
