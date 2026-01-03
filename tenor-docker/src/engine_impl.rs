use crate::client::DockerClient;
use crate::mapper::*;
use async_trait::async_trait;
use futures::stream;
use std::time::Duration;
use tenor_core::*;

pub struct DockerEngine {
    client: DockerClient,
}

impl DockerEngine {
    pub fn new(client: DockerClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl Engine for DockerEngine {
    async fn list_containers(&self, filter: ContainerFilter) -> EngineResult<Vec<Container>> {
        let mut query_params = vec!["all=true".to_string()];

        if let Some(state) = filter.state {
            let status_filter = match state {
                ContainerState::Running => "running",
                ContainerState::Exited => "exited",
                ContainerState::Paused => "paused",
                ContainerState::Restarting => "restarting",
                ContainerState::Dead => "dead",
                ContainerState::Unknown => "",
            };
            if !status_filter.is_empty() {
                query_params.push(format!("filters={{\"status\":[\"{}\"]}}", status_filter));
            }
        }

        let path = if query_params.is_empty() {
            "/containers/json".to_string()
        } else {
            format!("/containers/json?{}", query_params.join("&"))
        };

        let containers: Vec<DockerContainer> = self.client.get(&path).await?;
        Ok(containers.into_iter().map(|c| c.into_domain()).collect())
    }

    async fn inspect_container(&self, id: &ContainerId) -> EngineResult<ContainerDetail> {
        let path = format!("/containers/{}/json", id.as_ref());
        let detail: DockerContainerDetail = self.client.get(&path).await?;
        Ok(detail.into_domain())
    }

    async fn start_container(&self, id: &ContainerId) -> EngineResult<()> {
        let path = format!("/containers/{}/start", id.as_ref());
        self.client.post_no_response(&path).await
    }

    async fn stop_container(
        &self,
        id: &ContainerId,
        timeout: Option<Duration>,
    ) -> EngineResult<()> {
        let path = if let Some(t) = timeout {
            format!("/containers/{}/stop?t={}", id.as_ref(), t.as_secs())
        } else {
            format!("/containers/{}/stop", id.as_ref())
        };
        self.client.post_no_response(&path).await
    }

    async fn restart_container(
        &self,
        id: &ContainerId,
        timeout: Option<Duration>,
    ) -> EngineResult<()> {
        let path = if let Some(t) = timeout {
            format!("/containers/{}/restart?t={}", id.as_ref(), t.as_secs())
        } else {
            format!("/containers/{}/restart", id.as_ref())
        };
        self.client.post_no_response(&path).await
    }

    async fn delete_container(
        &self,
        id: &ContainerId,
        opts: DeleteContainerOpts,
    ) -> EngineResult<()> {
        let mut query_params = vec![];
        if opts.force {
            query_params.push("force=true");
        }
        if opts.remove_volumes {
            query_params.push("v=true");
        }

        let path = if query_params.is_empty() {
            format!("/containers/{}", id.as_ref())
        } else {
            format!("/containers/{}?{}", id.as_ref(), query_params.join("&"))
        };

        self.client.delete(&path).await
    }

    async fn stream_logs(&self, _id: &ContainerId, _opts: LogOpts) -> EngineResult<BoxedLogStream> {
        // TODO: Implement log streaming
        // For now, return empty stream
        Ok(Box::pin(stream::empty()))
    }

    async fn stream_stats(&self, _id: &ContainerId) -> EngineResult<BoxedStatsStream> {
        // TODO: Implement stats streaming
        Ok(Box::pin(stream::empty()))
    }

    async fn create_exec(&self, _id: &ContainerId, _spec: ExecSpec) -> EngineResult<ExecHandle> {
        // TODO: Implement exec
        Err(EngineError::user_actionable(
            "Exec not yet implemented",
            None,
        ))
    }

    async fn list_images(&self, _filter: ImageFilter) -> EngineResult<Vec<Image>> {
        let path = "/images/json";
        let images: Vec<DockerImage> = self.client.get(path).await?;
        Ok(images.into_iter().map(|i| i.into_domain()).collect())
    }

    async fn inspect_image(&self, id: &ImageId) -> EngineResult<ImageDetail> {
        let path = format!("/images/{}/json", id.as_ref());
        let image: DockerImageInspect = self.client.get(&path).await?;
        Ok(image.into_domain())
    }

    async fn remove_image(&self, id: &ImageId, force: bool) -> EngineResult<()> {
        let path = if force {
            format!("/images/{}?force=true", id.as_ref())
        } else {
            format!("/images/{}", id.as_ref())
        };
        self.client.delete(&path).await
    }

    async fn pull_image(&self, reference: &str) -> EngineResult<()> {
        let path = format!("/images/create?fromImage={}", reference);
        self.client.post_no_response(&path).await
    }

    async fn list_volumes(&self, _filter: VolumeFilter) -> EngineResult<Vec<Volume>> {
        let path = "/volumes";
        let response: DockerVolumeList = self.client.get(path).await?;
        Ok(response
            .volumes
            .unwrap_or_default()
            .into_iter()
            .map(|v| v.into_domain())
            .collect())
    }

    async fn inspect_volume(&self, name: &VolumeName) -> EngineResult<VolumeDetail> {
        let path = format!("/volumes/{}", name.as_ref());
        let volume: DockerVolumeInspect = self.client.get(&path).await?;
        Ok(volume.into_domain())
    }

    async fn remove_volume(&self, name: &VolumeName, force: bool) -> EngineResult<()> {
        let path = if force {
            format!("/volumes/{}?force=true", name.as_ref())
        } else {
            format!("/volumes/{}", name.as_ref())
        };
        self.client.delete(&path).await
    }

    async fn list_networks(&self, _filter: NetworkFilter) -> EngineResult<Vec<Network>> {
        let path = "/networks";
        let networks: Vec<DockerNetworkInfo> = self.client.get(path).await?;
        Ok(networks.into_iter().map(|n| n.into_domain()).collect())
    }

    async fn inspect_network(&self, id: &NetworkId) -> EngineResult<NetworkDetail> {
        let path = format!("/networks/{}", id.as_ref());
        let network: DockerNetworkInspect = self.client.get(&path).await?;
        Ok(network.into_domain())
    }

    async fn remove_network(&self, id: &NetworkId) -> EngineResult<()> {
        let path = format!("/networks/{}", id.as_ref());
        self.client.delete(&path).await
    }

    async fn ping(&self) -> EngineResult<()> {
        let _: serde_json::Value = self.client.get("/_ping").await?;
        Ok(())
    }

    async fn engine_info(&self) -> EngineResult<EngineInfo> {
        #[derive(serde::Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct VersionResponse {
            version: String,
            api_version: String,
            os: String,
            arch: String,
        }

        let info: VersionResponse = self.client.get("/version").await?;
        Ok(EngineInfo {
            version: info.version,
            api_version: info.api_version,
            os: info.os,
            arch: info.arch,
        })
    }
}
