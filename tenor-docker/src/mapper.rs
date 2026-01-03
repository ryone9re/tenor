/// Mapper module for converting Docker API DTOs to domain models
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::BTreeMap;
use tenor_core::*;

/// Docker API Container response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockerContainer {
    pub id: String,
    pub names: Vec<String>,
    pub image: String,
    pub state: String,
    pub status: String,
    pub created: i64,
    #[serde(default)]
    pub labels: BTreeMap<String, String>,
    #[serde(default)]
    pub ports: Vec<DockerPort>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockerPort {
    #[serde(rename = "PrivatePort")]
    pub private_port: u16,
    #[serde(rename = "PublicPort")]
    pub public_port: Option<u16>,
    #[serde(rename = "Type")]
    pub port_type: String,
    #[serde(rename = "IP")]
    pub ip: Option<String>,
}

/// Docker API Container inspect response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockerContainerDetail {
    pub id: String,
    pub name: String,
    pub config: DockerConfig,
    pub state: DockerState,
    pub created: String,
    #[serde(default)]
    pub mounts: Vec<DockerMount>,
    pub network_settings: DockerNetworkSettings,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockerConfig {
    pub image: String,
    #[serde(default)]
    pub cmd: Vec<String>,
    #[serde(default)]
    pub entrypoint: Vec<String>,
    #[serde(default)]
    pub env: Vec<String>,
    #[serde(default)]
    pub labels: BTreeMap<String, String>,
    #[serde(default)]
    #[allow(dead_code)] // Will be used for port mapping in future
    pub exposed_ports: Option<BTreeMap<String, serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockerState {
    pub status: String,
    pub running: bool,
    pub paused: bool,
    pub restarting: bool,
    pub dead: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockerMount {
    pub source: String,
    pub destination: String,
    pub mode: String,
    #[serde(rename = "RW")]
    pub rw: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockerNetworkSettings {
    #[serde(default)]
    pub networks: BTreeMap<String, DockerNetwork>,
    #[serde(rename = "IPAddress")]
    pub ip_address: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)] // Fields used for JSON deserialization
pub struct DockerNetwork {
    #[serde(rename = "NetworkID")]
    pub network_id: String,
    #[serde(rename = "IPAddress")]
    pub ip_address: Option<String>,
}

/// Docker API Image response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DockerImage {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(default)]
    pub repo_tags: Vec<String>,
    pub size: u64,
    pub created: i64,
    #[serde(default)]
    pub labels: Option<BTreeMap<String, String>>,
}

/// Mappers
impl DockerContainer {
    pub fn into_domain(self) -> Container {
        let name = self
            .names
            .first()
            .map(|n| n.trim_start_matches('/').to_string())
            .unwrap_or_default();

        let state = match self.state.to_lowercase().as_str() {
            "running" => ContainerState::Running,
            "exited" => ContainerState::Exited,
            "paused" => ContainerState::Paused,
            "restarting" => ContainerState::Restarting,
            "dead" => ContainerState::Dead,
            _ => ContainerState::Unknown,
        };

        let ports = self
            .ports
            .into_iter()
            .map(|p| PortMapping {
                container_port: p.private_port,
                host_port: p.public_port,
                host_ip: p.ip,
                protocol: if p.port_type == "udp" {
                    PortProtocol::Udp
                } else {
                    PortProtocol::Tcp
                },
            })
            .collect();

        Container {
            id: ContainerId(self.id),
            name,
            image: self.image,
            state,
            status: self.status,
            created_at: DateTime::from_timestamp(self.created, 0).unwrap_or_default(),
            labels: self.labels,
            ports,
        }
    }
}

impl DockerContainerDetail {
    pub fn into_domain(self) -> ContainerDetail {
        let state = if self.state.running {
            ContainerState::Running
        } else if self.state.paused {
            ContainerState::Paused
        } else if self.state.restarting {
            ContainerState::Restarting
        } else if self.state.dead {
            ContainerState::Dead
        } else {
            ContainerState::Exited
        };

        let mounts = self
            .mounts
            .into_iter()
            .map(|m| Mount {
                source: m.source,
                destination: m.destination,
                mode: m.mode,
                rw: m.rw,
            })
            .collect();

        let networks = self.network_settings.networks.keys().cloned().collect();

        ContainerDetail {
            id: ContainerId(self.id),
            name: self.name.trim_start_matches('/').to_string(),
            image: self.config.image,
            state,
            status: self.state.status,
            created_at: DateTime::parse_from_rfc3339(&self.created)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_default(),
            labels: self.config.labels,
            ports: vec![], // Would need to parse from config.exposed_ports
            command: self.config.cmd,
            entrypoint: self.config.entrypoint,
            env: self.config.env,
            mounts,
            network_settings: NetworkSettings {
                networks,
                ip_address: self.network_settings.ip_address,
            },
        }
    }
}

impl DockerImage {
    pub fn into_domain(self) -> Image {
        Image {
            id: ImageId(self.id),
            repo_tags: self.repo_tags,
            size: self.size,
            created_at: DateTime::from_timestamp(self.created, 0).unwrap_or_default(),
            labels: self.labels.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_docker_container_to_domain() {
        let docker_container = DockerContainer {
            id: "abc123".to_string(),
            names: vec!["/test-container".to_string()],
            image: "nginx:latest".to_string(),
            state: "running".to_string(),
            status: "Up 5 minutes".to_string(),
            created: 1700000000,
            labels: BTreeMap::new(),
            ports: vec![],
        };

        let container = docker_container.into_domain();
        assert_eq!(container.id.0, "abc123");
        assert_eq!(container.name, "test-container");
        assert_eq!(container.image, "nginx:latest");
        assert!(matches!(container.state, ContainerState::Running));
    }

    #[test]
    fn test_docker_container_state_mapping() {
        let states = vec![
            ("running", ContainerState::Running),
            ("exited", ContainerState::Exited),
            ("paused", ContainerState::Paused),
            ("restarting", ContainerState::Restarting),
            ("dead", ContainerState::Dead),
            ("unknown", ContainerState::Unknown),
        ];

        for (state_str, expected_state) in states {
            let docker_container = DockerContainer {
                id: "test".to_string(),
                names: vec!["/test".to_string()],
                image: "test:latest".to_string(),
                state: state_str.to_string(),
                status: "test".to_string(),
                created: 1700000000,
                labels: BTreeMap::new(),
                ports: vec![],
            };
            let container = docker_container.into_domain();
            assert!(
                matches!(container.state, ref s if std::mem::discriminant(s) == std::mem::discriminant(&expected_state)),
                "State '{}' should map to {:?}, got {:?}",
                state_str,
                expected_state,
                container.state
            );
        }
    }

    #[test]
    fn test_docker_image_to_domain() {
        let mut labels = BTreeMap::new();
        labels.insert("version".to_string(), "1.0".to_string());

        let docker_image = DockerImage {
            id: "sha256:abc123".to_string(),
            repo_tags: vec!["nginx:latest".to_string(), "nginx:1.0".to_string()],
            size: 1024 * 1024,
            created: 1700000000,
            labels: Some(labels),
        };

        let image = docker_image.into_domain();
        assert_eq!(image.id.0, "sha256:abc123");
        assert_eq!(image.repo_tags.len(), 2);
        assert_eq!(image.size, 1024 * 1024);
        assert_eq!(image.labels.get("version"), Some(&"1.0".to_string()));
    }

    #[test]
    fn test_docker_container_detail_to_domain() {
        let docker_detail = DockerContainerDetail {
            id: "abc123".to_string(),
            name: "/test-container".to_string(),
            created: "2024-01-01T00:00:00.000000000Z".to_string(),
            config: DockerConfig {
                image: "nginx:latest".to_string(),
                cmd: vec!["nginx".to_string()],
                entrypoint: vec!["/docker-entrypoint.sh".to_string()],
                env: vec!["PATH=/usr/bin".to_string()],
                labels: BTreeMap::new(),
                exposed_ports: None,
            },
            state: DockerState {
                status: "running".to_string(),
                running: true,
                paused: false,
                restarting: false,
                dead: false,
            },
            mounts: vec![],
            network_settings: DockerNetworkSettings {
                networks: BTreeMap::new(),
                ip_address: Some("172.17.0.2".to_string()),
            },
        };

        let detail = docker_detail.into_domain();
        assert_eq!(detail.id.0, "abc123");
        assert_eq!(detail.name, "test-container");
        assert_eq!(detail.image, "nginx:latest");
        assert!(matches!(detail.state, ContainerState::Running));
        assert_eq!(detail.command, vec!["nginx".to_string()]);
    }

    #[test]
    fn test_docker_port_to_domain() {
        let docker_port = DockerPort {
            private_port: 80,
            public_port: Some(8080),
            port_type: "tcp".to_string(),
            ip: Some("0.0.0.0".to_string()),
        };

        // Port is converted inside into_domain, so test via container
        let docker_container = DockerContainer {
            id: "test".to_string(),
            names: vec!["/test".to_string()],
            image: "nginx:latest".to_string(),
            state: "running".to_string(),
            status: "Up".to_string(),
            created: 1700000000,
            labels: BTreeMap::new(),
            ports: vec![docker_port],
        };

        let container = docker_container.into_domain();
        assert_eq!(container.ports.len(), 1);
        assert_eq!(container.ports[0].container_port, 80);
        assert_eq!(container.ports[0].host_port, Some(8080));
        assert_eq!(container.ports[0].protocol, PortProtocol::Tcp);
        assert_eq!(container.ports[0].host_ip, Some("0.0.0.0".to_string()));
    }
}
