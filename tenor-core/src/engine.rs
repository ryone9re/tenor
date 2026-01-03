use crate::domain::*;
use crate::error::EngineResult;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures::Stream;
use std::pin::Pin;
use std::time::Duration;

/// Log stream event
#[derive(Debug, Clone)]
pub struct LogEvent {
    pub timestamp: Option<DateTime<Utc>>,
    pub stream: LogStream,
    pub line: String,
}

/// Log stream type (stdout or stderr)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogStream {
    Stdout,
    Stderr,
}

/// Log options
#[derive(Debug, Clone, Default)]
pub struct LogOpts {
    pub follow: bool,
    pub since: Option<DateTime<Utc>>,
    pub timestamps: bool,
    pub tail: Option<usize>,
}

/// Stats event
#[derive(Debug, Clone)]
pub struct StatsEvent {
    pub timestamp: DateTime<Utc>,
    pub cpu_percent: f64,
    pub memory_usage: u64,
    pub memory_limit: u64,
    pub network_rx: u64,
    pub network_tx: u64,
    pub block_read: u64,
    pub block_write: u64,
}

/// Exec specification
#[derive(Debug, Clone)]
pub struct ExecSpec {
    pub cmd: Vec<String>,
    pub tty: bool,
    pub attach_stdin: bool,
    pub env: Vec<String>,
    pub user: Option<String>,
}

/// Exec handle (used to start an exec session)
#[derive(Debug, Clone)]
pub struct ExecHandle {
    pub id: String,
}

/// Type alias for async streams
pub type BoxedLogStream = Pin<Box<dyn Stream<Item = EngineResult<LogEvent>> + Send>>;
pub type BoxedStatsStream = Pin<Box<dyn Stream<Item = EngineResult<StatsEvent>> + Send>>;

/// Engine information
#[derive(Debug, Clone)]
pub struct EngineInfo {
    pub version: String,
    pub api_version: String,
    pub os: String,
    pub arch: String,
}

/// Main Engine trait for container operations
#[async_trait]
pub trait Engine: Send + Sync {
    // Container operations
    async fn list_containers(&self, filter: ContainerFilter) -> EngineResult<Vec<Container>>;
    async fn inspect_container(&self, id: &ContainerId) -> EngineResult<ContainerDetail>;
    async fn start_container(&self, id: &ContainerId) -> EngineResult<()>;
    async fn stop_container(&self, id: &ContainerId, timeout: Option<Duration>)
        -> EngineResult<()>;
    async fn restart_container(
        &self,
        id: &ContainerId,
        timeout: Option<Duration>,
    ) -> EngineResult<()>;
    async fn delete_container(
        &self,
        id: &ContainerId,
        opts: DeleteContainerOpts,
    ) -> EngineResult<()>;

    // Logs
    async fn stream_logs(&self, id: &ContainerId, opts: LogOpts) -> EngineResult<BoxedLogStream>;

    // Stats (optional for v0.1)
    async fn stream_stats(&self, id: &ContainerId) -> EngineResult<BoxedStatsStream>;

    // Exec
    async fn create_exec(&self, id: &ContainerId, spec: ExecSpec) -> EngineResult<ExecHandle>;

    // Image operations
    async fn list_images(&self, filter: ImageFilter) -> EngineResult<Vec<Image>>;
    async fn inspect_image(&self, id: &ImageId) -> EngineResult<ImageDetail>;
    async fn remove_image(&self, id: &ImageId, force: bool) -> EngineResult<()>;
    async fn pull_image(&self, reference: &str) -> EngineResult<()>;

    // Volume operations
    async fn list_volumes(&self, filter: VolumeFilter) -> EngineResult<Vec<Volume>>;
    async fn inspect_volume(&self, name: &VolumeName) -> EngineResult<VolumeDetail>;
    async fn remove_volume(&self, name: &VolumeName, force: bool) -> EngineResult<()>;

    // Network operations
    async fn list_networks(&self, filter: NetworkFilter) -> EngineResult<Vec<Network>>;
    async fn inspect_network(&self, id: &NetworkId) -> EngineResult<NetworkDetail>;
    async fn remove_network(&self, id: &NetworkId) -> EngineResult<()>;

    // System operations
    async fn ping(&self) -> EngineResult<()>;
    async fn engine_info(&self) -> EngineResult<EngineInfo>;
}
