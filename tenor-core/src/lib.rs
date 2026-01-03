pub mod domain;
pub mod engine;
pub mod error;

pub use domain::*;
pub use engine::*;
pub use error::*;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_container_id_display() {
        let id = ContainerId("abc123".to_string());
        assert_eq!(format!("{}", id), "abc123");
    }

    #[test]
    fn test_container_id_as_ref() {
        let id = ContainerId("abc123".to_string());
        assert_eq!(id.as_ref(), "abc123");
    }

    #[test]
    fn test_image_id_display() {
        let id = ImageId("sha256:abc123".to_string());
        assert_eq!(format!("{}", id), "sha256:abc123");
    }

    #[test]
    fn test_container_state_transitions() {
        let states = [
            ContainerState::Running,
            ContainerState::Exited,
            ContainerState::Paused,
            ContainerState::Restarting,
            ContainerState::Dead,
            ContainerState::Unknown,
        ];
        assert_eq!(states.len(), 6);
    }

    #[test]
    fn test_container_filter_default() {
        let filter = ContainerFilter::default();
        assert!(filter.state.is_none());
        assert!(filter.name.is_none());
        assert!(filter.labels.is_empty());
    }

    #[test]
    fn test_delete_container_opts_default() {
        let opts = DeleteContainerOpts::default();
        assert!(!opts.force);
        assert!(!opts.remove_volumes);
    }

    #[test]
    fn test_exec_spec_creation() {
        let spec = ExecSpec {
            cmd: vec!["sh".to_string(), "-c".to_string(), "ls".to_string()],
            attach_stdin: true,
            tty: false,
            env: vec!["PATH=/usr/bin".to_string()],
            user: None,
        };
        assert_eq!(spec.cmd.len(), 3);
        assert!(spec.attach_stdin);
        assert!(!spec.tty);
    }

    #[test]
    fn test_port_mapping() {
        let port = PortMapping {
            container_port: 80,
            host_port: Some(8080),
            host_ip: None,
            protocol: PortProtocol::Tcp,
        };
        assert_eq!(port.container_port, 80);
        assert_eq!(port.host_port, Some(8080));
        assert_eq!(port.protocol, PortProtocol::Tcp);
    }

    #[test]
    fn test_image() {
        let now = Utc::now();
        let mut labels = std::collections::BTreeMap::new();
        labels.insert("version".to_string(), "1.0".to_string());

        let image = Image {
            id: ImageId("sha256:abc".to_string()),
            repo_tags: vec!["nginx:latest".to_string()],
            size: 1024 * 1024,
            created_at: now,
            labels,
        };
        assert_eq!(image.repo_tags.len(), 1);
        assert_eq!(image.size, 1024 * 1024);
    }

    #[test]
    fn test_engine_error_user_actionable() {
        let err = EngineError::user_actionable(
            "Container not found",
            Some("Check container ID".to_string()),
        );
        assert!(matches!(err, EngineError::UserActionable { .. }));
    }

    #[test]
    fn test_engine_error_retryable() {
        let err = EngineError::retryable("Network timeout");
        assert!(matches!(err, EngineError::Retryable { .. }));
    }

    #[test]
    fn test_engine_error_bug() {
        let err = EngineError::bug("Unexpected state");
        assert!(matches!(err, EngineError::Bug { .. }));
    }
}
