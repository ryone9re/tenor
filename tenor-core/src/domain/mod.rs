mod container;
mod image;
mod network;
mod volume;

pub use container::*;
pub use image::*;
pub use network::*;
pub use volume::*;

use chrono::{DateTime, Utc};
use std::collections::BTreeMap;

/// Common labels type used across domain models
pub type Labels = BTreeMap<String, String>;

/// Common timestamp type
pub type Timestamp = DateTime<Utc>;
