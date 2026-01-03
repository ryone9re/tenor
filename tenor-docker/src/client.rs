use crate::ConnectionTarget;
use http_body_util::{BodyExt, Empty, Full};
use hyper::body::Bytes;
use hyper::{Request, StatusCode};
use hyper_util::client::legacy::Client;
use serde::de::DeserializeOwned;
use std::path::PathBuf;
use tenor_core::EngineError;

#[cfg(unix)]
use hyperlocal_next::{UnixClientExt, Uri};

/// Docker API client
#[derive(Clone)]
pub struct DockerClient {
    socket_path: PathBuf,
}

impl DockerClient {
    pub fn new(target: ConnectionTarget) -> Result<Self, EngineError> {
        match target {
            ConnectionTarget::UnixSocket(path) => Ok(Self { socket_path: path }),
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, EngineError> {
        #[cfg(unix)]
        {
            let client = Client::unix();
            let uri: hyper::Uri = Uri::new(&self.socket_path, path).into();

            let req = Request::builder()
                .uri(uri)
                .body(Empty::<Bytes>::new())
                .map_err(|e| EngineError::bug_with_source("Failed to build request", e))?;

            let response = client.request(req).await.map_err(Self::map_hyper_error)?;

            let status = response.status();
            let body_bytes = response
                .into_body()
                .collect()
                .await
                .map_err(|e| EngineError::retryable_with_source("Failed to read response body", e))?
                .to_bytes();

            if !status.is_success() {
                let body = String::from_utf8_lossy(&body_bytes);
                return Err(Self::handle_error_response(status, &body));
            }

            serde_json::from_slice(&body_bytes)
                .map_err(|e| EngineError::bug_with_source("Failed to parse response", e))
        }

        #[cfg(not(unix))]
        {
            Err(EngineError::user_actionable(
                "Unix sockets are not supported on this platform",
                Some("Use TCP connection instead".to_string()),
            ))
        }
    }

    pub async fn post<T: DeserializeOwned>(
        &self,
        path: &str,
        body: Option<String>,
    ) -> Result<T, EngineError> {
        #[cfg(unix)]
        {
            let client = Client::unix();
            let uri: hyper::Uri = Uri::new(&self.socket_path, path).into();

            let body_data = body.unwrap_or_default();
            let req = Request::builder()
                .method("POST")
                .uri(uri)
                .header("Content-Type", "application/json")
                .body(Full::new(Bytes::from(body_data)))
                .map_err(|e| EngineError::bug_with_source("Failed to build request", e))?;

            let response = client.request(req).await.map_err(Self::map_hyper_error)?;

            let status = response.status();
            let body_bytes = response
                .into_body()
                .collect()
                .await
                .map_err(|e| EngineError::retryable_with_source("Failed to read response body", e))?
                .to_bytes();

            if !status.is_success() {
                let body = String::from_utf8_lossy(&body_bytes);
                return Err(Self::handle_error_response(status, &body));
            }

            serde_json::from_slice(&body_bytes)
                .map_err(|e| EngineError::bug_with_source("Failed to parse response", e))
        }

        #[cfg(not(unix))]
        {
            Err(EngineError::user_actionable(
                "Unix sockets are not supported on this platform",
                Some("Use TCP connection instead".to_string()),
            ))
        }
    }

    pub async fn post_no_response(&self, path: &str) -> Result<(), EngineError> {
        #[cfg(unix)]
        {
            let client = Client::unix();
            let uri: hyper::Uri = Uri::new(&self.socket_path, path).into();

            let req = Request::builder()
                .method("POST")
                .uri(uri)
                .body(Empty::<Bytes>::new())
                .map_err(|e| EngineError::bug_with_source("Failed to build request", e))?;

            let response = client.request(req).await.map_err(Self::map_hyper_error)?;

            let status = response.status();

            if !status.is_success() {
                let body_bytes = response
                    .into_body()
                    .collect()
                    .await
                    .map_err(|e| {
                        EngineError::retryable_with_source("Failed to read response body", e)
                    })?
                    .to_bytes();
                let body = String::from_utf8_lossy(&body_bytes);
                return Err(Self::handle_error_response(status, &body));
            }

            Ok(())
        }

        #[cfg(not(unix))]
        {
            Err(EngineError::user_actionable(
                "Unix sockets are not supported on this platform",
                Some("Use TCP connection instead".to_string()),
            ))
        }
    }

    pub async fn delete(&self, path: &str) -> Result<(), EngineError> {
        #[cfg(unix)]
        {
            let client = Client::unix();
            let uri: hyper::Uri = Uri::new(&self.socket_path, path).into();

            let req = Request::builder()
                .method("DELETE")
                .uri(uri)
                .body(Empty::<Bytes>::new())
                .map_err(|e| EngineError::bug_with_source("Failed to build request", e))?;

            let response = client.request(req).await.map_err(Self::map_hyper_error)?;

            let status = response.status();

            if !status.is_success() {
                let body_bytes = response
                    .into_body()
                    .collect()
                    .await
                    .map_err(|e| {
                        EngineError::retryable_with_source("Failed to read response body", e)
                    })?
                    .to_bytes();
                let body = String::from_utf8_lossy(&body_bytes);
                return Err(Self::handle_error_response(status, &body));
            }

            Ok(())
        }

        #[cfg(not(unix))]
        {
            Err(EngineError::user_actionable(
                "Unix sockets are not supported on this platform",
                Some("Use TCP connection instead".to_string()),
            ))
        }
    }

    fn map_hyper_error(err: hyper_util::client::legacy::Error) -> EngineError {
        EngineError::retryable_with_source("Network error", err)
    }

    fn handle_error_response(status: StatusCode, body: &str) -> EngineError {
        match status.as_u16() {
            404 => EngineError::user_actionable(format!("Resource not found: {}", body), None),
            403 | 401 => EngineError::user_actionable(
                "Permission denied",
                Some("Check Docker socket permissions or authentication".to_string()),
            ),
            409 => EngineError::user_actionable(format!("Conflict: {}", body), None),
            500 => EngineError::retryable(format!("Docker daemon error: {}", body)),
            _ => EngineError::bug(format!("Unexpected status {}: {}", status, body)),
        }
    }
}
