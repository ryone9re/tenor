use serde::Deserialize;
use std::path::PathBuf;
use tenor_core::EngineError;

/// Docker context configuration
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockerContext {
    pub name: String,
    pub endpoints: ContextEndpoints,
}

#[derive(Debug, Deserialize)]
pub struct ContextEndpoints {
    pub docker: DockerEndpoint,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockerEndpoint {
    pub host: String,
}

/// Get the current Docker context configuration
pub async fn get_current_context() -> Result<DockerContext, EngineError> {
    let output = tokio::process::Command::new("docker")
        .args(["context", "inspect"])
        .output()
        .await
        .map_err(|e| {
            EngineError::user_actionable_with_source(
                "Failed to execute docker context inspect",
                Some("Make sure Docker CLI is installed and in PATH".to_string()),
                e,
            )
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(EngineError::user_actionable(
            format!("Docker context inspect failed: {}", stderr),
            Some("Check if Docker is installed and configured".to_string()),
        ));
    }

    let contexts: Vec<DockerContext> = serde_json::from_slice(&output.stdout)
        .map_err(|e| EngineError::bug_with_source("Failed to parse docker context output", e))?;

    contexts
        .into_iter()
        .next()
        .ok_or_else(|| EngineError::user_actionable("No Docker context found", None))
}

/// Parse Docker host URL to extract socket path
pub fn parse_host_to_socket(host: &str) -> Result<PathBuf, EngineError> {
    if let Some(path) = host.strip_prefix("unix://") {
        Ok(PathBuf::from(path))
    } else if host.starts_with("tcp://") || host.starts_with("http://") {
        Err(EngineError::user_actionable(
            format!("TCP connections not yet supported: {}", host),
            Some("Only Unix sockets are currently supported".to_string()),
        ))
    } else {
        // Assume it's a direct path
        Ok(PathBuf::from(host))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_unix_socket() {
        let result = parse_host_to_socket("unix:///var/run/docker.sock").unwrap();
        assert_eq!(result, PathBuf::from("/var/run/docker.sock"));
    }

    #[test]
    fn test_parse_direct_path() {
        let result = parse_host_to_socket("/var/run/docker.sock").unwrap();
        assert_eq!(result, PathBuf::from("/var/run/docker.sock"));
    }

    #[test]
    fn test_parse_tcp_fails() {
        let result = parse_host_to_socket("tcp://localhost:2375");
        assert!(result.is_err());
    }
}
