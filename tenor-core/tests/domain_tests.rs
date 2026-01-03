use tenor_core::{ContainerId, ContainerState, ImageId, NetworkId, PortProtocol, VolumeName};

#[test]
fn test_container_state_display() {
    assert_eq!(ContainerState::Running.to_string(), "Running");
    assert_eq!(ContainerState::Exited.to_string(), "Exited");
    assert_eq!(ContainerState::Paused.to_string(), "Paused");
    assert_eq!(ContainerState::Restarting.to_string(), "Restarting");
    assert_eq!(ContainerState::Dead.to_string(), "Dead");
    assert_eq!(ContainerState::Unknown.to_string(), "Unknown");
}

#[test]
fn test_port_protocol_display() {
    assert_eq!(PortProtocol::Tcp.to_string(), "tcp");
    assert_eq!(PortProtocol::Udp.to_string(), "udp");
}

#[test]
fn test_container_id_display() {
    let id = ContainerId::from("abc123".to_string());
    assert_eq!(id.to_string(), "abc123");
}

#[test]
fn test_container_id_as_ref() {
    let id = ContainerId::from("abc123".to_string());
    assert_eq!(id.as_ref(), "abc123");

    // Test empty string
    let empty_id = ContainerId::from("".to_string());
    assert_eq!(empty_id.as_ref(), "");

    // Test with xyzzy (mutation test pattern)
    let xyzzy_id = ContainerId::from("xyzzy".to_string());
    assert_eq!(xyzzy_id.as_ref(), "xyzzy");
}

#[test]
fn test_image_id_display() {
    let id = ImageId::from("sha256:abc123".to_string());
    assert_eq!(id.to_string(), "sha256:abc123");
}

#[test]
fn test_image_id_as_ref() {
    let id = ImageId::from("sha256:abc123".to_string());
    assert_eq!(id.as_ref(), "sha256:abc123");

    // Test empty string
    let empty_id = ImageId::from("".to_string());
    assert_eq!(empty_id.as_ref(), "");

    // Test with xyzzy
    let xyzzy_id = ImageId::from("xyzzy".to_string());
    assert_eq!(xyzzy_id.as_ref(), "xyzzy");
}

#[test]
fn test_network_id_display() {
    let id = NetworkId::from("net123".to_string());
    assert_eq!(id.to_string(), "net123");
}

#[test]
fn test_network_id_as_ref() {
    let id = NetworkId::from("net123".to_string());
    assert_eq!(id.as_ref(), "net123");

    // Test empty string
    let empty_id = NetworkId::from("".to_string());
    assert_eq!(empty_id.as_ref(), "");

    // Test with xyzzy
    let xyzzy_id = NetworkId::from("xyzzy".to_string());
    assert_eq!(xyzzy_id.as_ref(), "xyzzy");
}

#[test]
fn test_volume_name_display() {
    let name = VolumeName::from("my-volume".to_string());
    assert_eq!(name.to_string(), "my-volume");
}

#[test]
fn test_volume_name_as_ref() {
    let name = VolumeName::from("my-volume".to_string());
    assert_eq!(name.as_ref(), "my-volume");

    // Test empty string
    let empty_name = VolumeName::from("".to_string());
    assert_eq!(empty_name.as_ref(), "");

    // Test with xyzzy
    let xyzzy_name = VolumeName::from("xyzzy".to_string());
    assert_eq!(xyzzy_name.as_ref(), "xyzzy");
}
