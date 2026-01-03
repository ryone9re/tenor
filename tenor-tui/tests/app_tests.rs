use chrono::Utc;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::BTreeMap;
use std::sync::Arc;
use tenor_core::{
    BoxedLogStream, BoxedStatsStream, Container, ContainerDetail, ContainerFilter, ContainerId,
    ContainerState, DeleteContainerOpts, Engine, EngineError, EngineInfo, EngineResult, ExecHandle,
    ExecSpec, Image, ImageDetail, ImageFilter, ImageId, LogOpts, Network, NetworkDetail,
    NetworkFilter, NetworkId, Volume, VolumeDetail, VolumeFilter, VolumeName,
};
use tenor_tui::app::{App, Tab};

// Mock Engine for testing
#[derive(Clone)]
struct MockEngine {
    containers: Vec<Container>,
    images: Vec<Image>,
    volumes: Vec<Volume>,
    networks: Vec<Network>,
}

impl MockEngine {
    fn new() -> Self {
        Self {
            containers: vec![
                Container {
                    id: ContainerId::from("test-container-1".to_string()),
                    name: "test1".to_string(),
                    image: "nginx:latest".to_string(),
                    state: ContainerState::Running,
                    status: "Up 2 hours".to_string(),
                    created_at: Utc::now(),
                    labels: BTreeMap::new(),
                    ports: vec![],
                },
                Container {
                    id: ContainerId::from("test-container-2".to_string()),
                    name: "test2".to_string(),
                    image: "redis:alpine".to_string(),
                    state: ContainerState::Exited,
                    status: "Exited (0) 5 minutes ago".to_string(),
                    created_at: Utc::now(),
                    labels: BTreeMap::new(),
                    ports: vec![],
                },
            ],
            images: vec![Image {
                id: ImageId::from("test-image-1".to_string()),
                repo_tags: vec!["nginx:latest".to_string()],
                size: 1024 * 1024 * 100,
                created_at: Utc::now(),
                labels: BTreeMap::new(),
            }],
            volumes: vec![Volume {
                name: VolumeName::from("test-volume-1".to_string()),
                driver: "local".to_string(),
                mountpoint: "/var/lib/docker/volumes/test-volume-1/_data".to_string(),
                labels: BTreeMap::new(),
            }],
            networks: vec![Network {
                id: NetworkId::from("test-network-1".to_string()),
                name: "bridge".to_string(),
                driver: "bridge".to_string(),
                scope: "local".to_string(),
                internal: false,
                labels: BTreeMap::new(),
            }],
        }
    }
}

#[async_trait::async_trait]
impl Engine for MockEngine {
    async fn list_containers(&self, _filter: ContainerFilter) -> EngineResult<Vec<Container>> {
        Ok(self.containers.clone())
    }

    async fn inspect_container(&self, _id: &ContainerId) -> EngineResult<ContainerDetail> {
        Err(EngineError::user_actionable("Not implemented", None))
    }

    async fn start_container(&self, _id: &ContainerId) -> EngineResult<()> {
        Ok(())
    }

    async fn stop_container(
        &self,
        _id: &ContainerId,
        _timeout: Option<std::time::Duration>,
    ) -> EngineResult<()> {
        Ok(())
    }

    async fn restart_container(
        &self,
        _id: &ContainerId,
        _timeout: Option<std::time::Duration>,
    ) -> EngineResult<()> {
        Ok(())
    }

    async fn delete_container(
        &self,
        _id: &ContainerId,
        _opts: DeleteContainerOpts,
    ) -> EngineResult<()> {
        Ok(())
    }

    async fn stream_logs(&self, _id: &ContainerId, _opts: LogOpts) -> EngineResult<BoxedLogStream> {
        Err(EngineError::user_actionable("Not implemented", None))
    }

    async fn stream_stats(&self, _id: &ContainerId) -> EngineResult<BoxedStatsStream> {
        Err(EngineError::user_actionable("Not implemented", None))
    }

    async fn create_exec(&self, _id: &ContainerId, _spec: ExecSpec) -> EngineResult<ExecHandle> {
        Err(EngineError::user_actionable("Not implemented", None))
    }

    async fn list_images(&self, _filter: ImageFilter) -> EngineResult<Vec<Image>> {
        Ok(self.images.clone())
    }

    async fn inspect_image(&self, _id: &ImageId) -> EngineResult<ImageDetail> {
        Err(EngineError::user_actionable("Not implemented", None))
    }

    async fn remove_image(&self, _id: &ImageId, _force: bool) -> EngineResult<()> {
        Ok(())
    }

    async fn pull_image(&self, _reference: &str) -> EngineResult<()> {
        Ok(())
    }

    async fn list_volumes(&self, _filter: VolumeFilter) -> EngineResult<Vec<Volume>> {
        Ok(self.volumes.clone())
    }

    async fn inspect_volume(&self, _name: &VolumeName) -> EngineResult<VolumeDetail> {
        Err(EngineError::user_actionable("Not implemented", None))
    }

    async fn remove_volume(&self, _name: &VolumeName, _force: bool) -> EngineResult<()> {
        Ok(())
    }

    async fn list_networks(&self, _filter: NetworkFilter) -> EngineResult<Vec<Network>> {
        Ok(self.networks.clone())
    }

    async fn inspect_network(&self, _id: &NetworkId) -> EngineResult<NetworkDetail> {
        Err(EngineError::user_actionable("Not implemented", None))
    }

    async fn remove_network(&self, _id: &NetworkId) -> EngineResult<()> {
        Ok(())
    }

    async fn ping(&self) -> EngineResult<()> {
        Ok(())
    }

    async fn engine_info(&self) -> EngineResult<EngineInfo> {
        Ok(EngineInfo {
            version: "test".to_string(),
            api_version: "1.0".to_string(),
            os: "test".to_string(),
            arch: "test".to_string(),
        })
    }
}

// Helper to create a test app
async fn create_test_app() -> App {
    let engine = Arc::new(MockEngine::new());
    App::with_engine(engine).await.unwrap()
}

// Helper to create a KeyEvent
fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::empty())
}

fn key_with_ctrl(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::CONTROL)
}

#[tokio::test]
async fn test_quit_with_q_key() {
    let mut app = create_test_app().await;
    assert!(!app.should_quit);

    app.handle_key(key(KeyCode::Char('q'))).await.unwrap();
    assert!(app.should_quit);
}

#[tokio::test]
async fn test_quit_with_ctrl_c() {
    let mut app = create_test_app().await;
    assert!(!app.should_quit);

    app.handle_key(key_with_ctrl(KeyCode::Char('c')))
        .await
        .unwrap();
    assert!(app.should_quit);
}

#[tokio::test]
async fn test_tab_navigation_with_tab_key() {
    let mut app = create_test_app().await;
    assert_eq!(app.current_tab, Tab::Containers);

    app.handle_key(key(KeyCode::Tab)).await.unwrap();
    assert_eq!(app.current_tab, Tab::Images);

    app.handle_key(key(KeyCode::Tab)).await.unwrap();
    assert_eq!(app.current_tab, Tab::Volumes);

    app.handle_key(key(KeyCode::Tab)).await.unwrap();
    assert_eq!(app.current_tab, Tab::Networks);

    app.handle_key(key(KeyCode::Tab)).await.unwrap();
    assert_eq!(app.current_tab, Tab::System);

    app.handle_key(key(KeyCode::Tab)).await.unwrap();
    assert_eq!(app.current_tab, Tab::Containers);
}

#[tokio::test]
async fn test_tab_navigation_with_backtab_key() {
    let mut app = create_test_app().await;
    assert_eq!(app.current_tab, Tab::Containers);

    app.handle_key(key(KeyCode::BackTab)).await.unwrap();
    assert_eq!(app.current_tab, Tab::System);

    app.handle_key(key(KeyCode::BackTab)).await.unwrap();
    assert_eq!(app.current_tab, Tab::Networks);
}

#[tokio::test]
async fn test_tab_navigation_with_number_keys() {
    let mut app = create_test_app().await;

    app.handle_key(key(KeyCode::Char('2'))).await.unwrap();
    assert_eq!(app.current_tab, Tab::Images);

    app.handle_key(key(KeyCode::Char('3'))).await.unwrap();
    assert_eq!(app.current_tab, Tab::Volumes);

    app.handle_key(key(KeyCode::Char('4'))).await.unwrap();
    assert_eq!(app.current_tab, Tab::Networks);

    app.handle_key(key(KeyCode::Char('5'))).await.unwrap();
    assert_eq!(app.current_tab, Tab::System);

    app.handle_key(key(KeyCode::Char('1'))).await.unwrap();
    assert_eq!(app.current_tab, Tab::Containers);
}

#[tokio::test]
async fn test_select_next_in_containers() {
    let mut app = create_test_app().await;
    assert_eq!(app.selected_container, 0);

    app.handle_key(key(KeyCode::Down)).await.unwrap();
    assert_eq!(app.selected_container, 1);

    app.handle_key(key(KeyCode::Char('j'))).await.unwrap();
    assert_eq!(app.selected_container, 0); // Wraps around
}

#[tokio::test]
async fn test_select_prev_in_containers() {
    let mut app = create_test_app().await;
    assert_eq!(app.selected_container, 0);

    app.handle_key(key(KeyCode::Up)).await.unwrap();
    assert_eq!(app.selected_container, 1); // Wraps to last

    app.handle_key(key(KeyCode::Char('k'))).await.unwrap();
    assert_eq!(app.selected_container, 0);
}

#[tokio::test]
async fn test_modal_navigation_left_right() {
    let mut app = create_test_app().await;

    // Show delete confirmation to open modal
    app.handle_key(key(KeyCode::Char('d'))).await.unwrap();
    assert!(app.modal.is_some());
    assert!(!app.modal_selected); // Default is "No"

    app.handle_key(key(KeyCode::Right)).await.unwrap();
    assert!(app.modal_selected); // Now "Yes"

    app.handle_key(key(KeyCode::Left)).await.unwrap();
    assert!(!app.modal_selected); // Back to "No"
}

#[tokio::test]
async fn test_modal_navigation_with_h_l() {
    let mut app = create_test_app().await;

    app.handle_key(key(KeyCode::Char('d'))).await.unwrap();
    assert!(app.modal.is_some());

    app.handle_key(key(KeyCode::Char('l'))).await.unwrap();
    assert!(app.modal_selected);

    app.handle_key(key(KeyCode::Char('h'))).await.unwrap();
    assert!(!app.modal_selected);
}

#[tokio::test]
async fn test_modal_navigation_with_tab() {
    let mut app = create_test_app().await;

    app.handle_key(key(KeyCode::Char('d'))).await.unwrap();
    assert!(!app.modal_selected);

    app.handle_key(key(KeyCode::Tab)).await.unwrap();
    assert!(app.modal_selected);

    app.handle_key(key(KeyCode::Tab)).await.unwrap();
    assert!(!app.modal_selected);
}

#[tokio::test]
async fn test_modal_close_with_esc() {
    let mut app = create_test_app().await;

    app.handle_key(key(KeyCode::Char('d'))).await.unwrap();
    assert!(app.modal.is_some());

    app.handle_key(key(KeyCode::Esc)).await.unwrap();
    assert!(app.modal.is_none());
}

#[tokio::test]
async fn test_modal_close_with_n() {
    let mut app = create_test_app().await;

    app.handle_key(key(KeyCode::Char('d'))).await.unwrap();
    assert!(app.modal.is_some());

    app.handle_key(key(KeyCode::Char('n'))).await.unwrap();
    assert!(app.modal.is_none());
}

#[tokio::test]
async fn test_refresh_with_r_key() {
    let mut app = create_test_app().await;
    let initial_count = app.containers.len();

    app.handle_key(key(KeyCode::Char('r'))).await.unwrap();
    assert_eq!(app.containers.len(), initial_count); // Should still work
}

#[tokio::test]
async fn test_tab_title() {
    assert_eq!(Tab::Containers.title(), "Containers");
    assert_eq!(Tab::Images.title(), "Images");
    assert_eq!(Tab::Volumes.title(), "Volumes");
    assert_eq!(Tab::Networks.title(), "Networks");
    assert_eq!(Tab::System.title(), "System");
}

#[tokio::test]
async fn test_tab_all() {
    let tabs = Tab::all();
    assert_eq!(tabs.len(), 5);
    assert_eq!(tabs[0], Tab::Containers);
    assert_eq!(tabs[1], Tab::Images);
    assert_eq!(tabs[2], Tab::Volumes);
    assert_eq!(tabs[3], Tab::Networks);
    assert_eq!(tabs[4], Tab::System);
}

#[tokio::test]
async fn test_start_container_operation() {
    let mut app = create_test_app().await;
    assert_eq!(app.selected_container, 0);

    // Start should succeed without error
    app.start_selected_container().await.unwrap();
}

#[tokio::test]
async fn test_stop_container_operation() {
    let mut app = create_test_app().await;

    // Stop should succeed without error
    app.stop_selected_container().await.unwrap();
}

#[tokio::test]
async fn test_restart_container_operation() {
    let mut app = create_test_app().await;

    // Restart should succeed without error
    app.restart_selected_container().await.unwrap();
}

#[tokio::test]
async fn test_select_next_wraps_around() {
    let mut app = create_test_app().await;
    assert_eq!(app.containers.len(), 2);
    assert_eq!(app.selected_container, 0);

    app.select_next();
    assert_eq!(app.selected_container, 1);

    app.select_next();
    assert_eq!(app.selected_container, 0); // Wraps to first
}

#[tokio::test]
async fn test_select_prev_wraps_around() {
    let mut app = create_test_app().await;
    assert_eq!(app.selected_container, 0);

    app.select_prev();
    assert_eq!(app.selected_container, 1); // Wraps to last

    app.select_prev();
    assert_eq!(app.selected_container, 0);
}

#[tokio::test]
async fn test_select_next_in_images_tab() {
    let mut app = create_test_app().await;
    app.current_tab = Tab::Images;
    app.refresh_images().await.unwrap();

    assert_eq!(app.selected_image, 0);
    assert_eq!(app.images.len(), 1);

    app.select_next();
    assert_eq!(app.selected_image, 0); // Wraps immediately with 1 item
}

#[tokio::test]
async fn test_select_next_in_volumes_tab() {
    let mut app = create_test_app().await;
    app.current_tab = Tab::Volumes;
    app.refresh_volumes().await.unwrap();

    assert_eq!(app.selected_volume, 0);
    assert_eq!(app.volumes.len(), 1);

    app.select_next();
    assert_eq!(app.selected_volume, 0);
}

#[tokio::test]
async fn test_select_next_in_networks_tab() {
    let mut app = create_test_app().await;
    app.current_tab = Tab::Networks;
    app.refresh_networks().await.unwrap();

    assert_eq!(app.selected_network, 0);
    assert_eq!(app.networks.len(), 1);

    app.select_next();
    assert_eq!(app.selected_network, 0);
}

#[tokio::test]
async fn test_refresh_updates_containers() {
    let mut app = create_test_app().await;
    assert!(!app.containers.is_empty());

    app.refresh_containers().await.unwrap();
    assert!(!app.containers.is_empty());
}

#[tokio::test]
async fn test_refresh_updates_images() {
    let mut app = create_test_app().await;

    app.refresh_images().await.unwrap();
    assert!(!app.images.is_empty());
}

#[tokio::test]
async fn test_refresh_updates_volumes() {
    let mut app = create_test_app().await;

    app.refresh_volumes().await.unwrap();
    assert!(!app.volumes.is_empty());
}

#[tokio::test]
async fn test_refresh_updates_networks() {
    let mut app = create_test_app().await;

    app.refresh_networks().await.unwrap();
    assert!(!app.networks.is_empty());
}

#[tokio::test]
async fn test_show_delete_confirmation_container() {
    let mut app = create_test_app().await;
    assert!(app.modal.is_none());

    app.show_delete_confirmation();
    assert!(app.modal.is_some());
}

#[tokio::test]
async fn test_show_delete_confirmation_image() {
    let mut app = create_test_app().await;
    app.current_tab = Tab::Images;
    app.refresh_images().await.unwrap();

    app.show_delete_confirmation();
    assert!(app.modal.is_some());
}

#[tokio::test]
async fn test_modal_selected_state() {
    let mut app = create_test_app().await;

    app.show_delete_confirmation();
    assert!(!app.modal_selected); // Default is No (false)

    assert!(!app.is_modal_confirm_selected());

    app.modal_selected = true;
    assert!(app.is_modal_confirm_selected());
}
