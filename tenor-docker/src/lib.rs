mod client;
mod context;
mod engine_impl;
mod mapper;

pub use client::DockerClient;
pub use context::{DockerContext, get_current_context, parse_host_to_socket};
pub use engine_impl::DockerEngine;

use std::path::PathBuf;

/// Connection target for Docker Engine
#[derive(Debug, Clone)]
pub enum ConnectionTarget {
    UnixSocket(PathBuf),
    // Future: Tcp { host: String, tls: Option<TlsConfig> }
}

impl ConnectionTarget {
    /// Create ConnectionTarget from current Docker context
    pub async fn from_current_context() -> Result<Self, tenor_core::EngineError> {
        let context = get_current_context().await?;
        let socket_path = parse_host_to_socket(&context.endpoints.docker.host)?;
        Ok(ConnectionTarget::UnixSocket(socket_path))
    }

    /// Create ConnectionTarget with fallback to default socket
    pub async fn from_context_or_default() -> Self {
        Self::from_current_context()
            .await
            .unwrap_or_else(|_| Self::default())
    }
}

impl Default for ConnectionTarget {
    fn default() -> Self {
        // Default socket path for macOS Docker Desktop / Linux
        #[cfg(target_os = "macos")]
        let socket = PathBuf::from("/var/run/docker.sock");

        #[cfg(target_os = "linux")]
        let socket = PathBuf::from("/var/run/docker.sock");

        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        let socket = PathBuf::from("/var/run/docker.sock");

        ConnectionTarget::UnixSocket(socket)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_target_default() {
        let target = ConnectionTarget::default();
        match target {
            ConnectionTarget::UnixSocket(path) => {
                assert_eq!(path, PathBuf::from("/var/run/docker.sock"));
            }
        }
    }

    #[test]
    fn test_connection_target_unix_socket() {
        let socket_path = PathBuf::from("/custom/docker.sock");
        let target = ConnectionTarget::UnixSocket(socket_path.clone());
        match target {
            ConnectionTarget::UnixSocket(path) => {
                assert_eq!(path, socket_path);
            }
        }
    }
}
