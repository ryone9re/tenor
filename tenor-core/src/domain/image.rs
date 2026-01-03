use super::{Labels, Timestamp};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Image identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ImageId(pub String);

impl fmt::Display for ImageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ImageId {
    fn from(s: String) -> Self {
        ImageId(s)
    }
}

impl AsRef<str> for ImageId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Image summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub id: ImageId,
    pub repo_tags: Vec<String>,
    pub size: u64,
    pub created_at: Timestamp,
    pub labels: Labels,
}

/// Detailed image information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageDetail {
    pub id: ImageId,
    pub repo_tags: Vec<String>,
    pub size: u64,
    pub created_at: Timestamp,
    pub labels: Labels,
    pub architecture: String,
    pub os: String,
}

/// Image filter options
#[derive(Debug, Clone, Default)]
pub struct ImageFilter {
    pub reference: Option<String>,
    pub dangling: Option<bool>,
}
