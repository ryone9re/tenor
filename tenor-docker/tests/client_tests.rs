use tenor_docker::{ConnectionTarget, DockerClient};

// HTTPエラーハンドリングのテストは内部メソッドなので、
// 実際のintegration testが必要。今回はplaceholderのみ

#[test]
fn test_client_creation() {
    use std::path::PathBuf;

    let target = ConnectionTarget::UnixSocket(PathBuf::from("/var/run/docker.sock"));

    // This should not panic
    let result = DockerClient::new(target);
    assert!(result.is_ok());
}
