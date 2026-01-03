use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::sync::Arc;
use tenor_core::{
    Container, ContainerDetail, ContainerFilter, ContainerId, Engine, Image, ImageDetail,
    ImageFilter, ImageId, Network, NetworkDetail, NetworkFilter, NetworkId, Volume, VolumeDetail,
    VolumeFilter, VolumeName,
};
use tenor_docker::{ConnectionTarget, DockerClient, DockerEngine};

use crate::components::ConfirmDialog;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Containers,
    Images,
    Volumes,
    Networks,
    System,
}

impl Tab {
    pub fn title(&self) -> &str {
        match self {
            Tab::Containers => "Containers",
            Tab::Images => "Images",
            Tab::Volumes => "Volumes",
            Tab::Networks => "Networks",
            Tab::System => "System",
        }
    }

    pub fn all() -> Vec<Tab> {
        vec![
            Tab::Containers,
            Tab::Images,
            Tab::Volumes,
            Tab::Networks,
            Tab::System,
        ]
    }

    pub fn next(&self) -> Tab {
        match self {
            Tab::Containers => Tab::Images,
            Tab::Images => Tab::Volumes,
            Tab::Volumes => Tab::Networks,
            Tab::Networks => Tab::System,
            Tab::System => Tab::Containers,
        }
    }

    pub fn prev(&self) -> Tab {
        match self {
            Tab::Containers => Tab::System,
            Tab::Images => Tab::Containers,
            Tab::Volumes => Tab::Images,
            Tab::Networks => Tab::Volumes,
            Tab::System => Tab::Networks,
        }
    }
}

#[derive(Debug, Clone)]
// TODO: Consider renaming to DeleteAction and removing Delete prefix from variants
// when we add non-delete modal actions (e.g., Restart, Exec, etc.)
#[allow(clippy::enum_variant_names)]
pub enum ModalAction {
    DeleteContainer(ContainerId),
    DeleteImage(ImageId),
    DeleteVolume(VolumeName),
    DeleteNetwork(NetworkId),
}

pub struct App {
    pub engine: Arc<dyn Engine>,
    pub current_tab: Tab,
    // Containers tab
    pub containers: Vec<Container>,
    pub selected_container: usize,
    pub show_details: bool,
    pub container_detail: Option<ContainerDetail>,
    // Images tab
    pub images: Vec<Image>,
    pub selected_image: usize,
    pub show_image_details: bool,
    pub image_detail: Option<ImageDetail>,
    // Volumes tab
    pub volumes: Vec<Volume>,
    pub selected_volume: usize,
    pub show_volume_details: bool,
    pub volume_detail: Option<VolumeDetail>,
    // Networks tab
    pub networks: Vec<Network>,
    pub selected_network: usize,
    pub show_network_details: bool,
    pub network_detail: Option<NetworkDetail>,
    // Global state
    pub should_quit: bool,
    pub modal: Option<(ConfirmDialog, ModalAction)>,
    pub modal_selected: bool, // true = confirm, false = cancel
}

impl App {
    pub async fn new() -> Result<Self> {
        // Try to get connection target from Docker context, fall back to default
        let target = ConnectionTarget::from_context_or_default().await;
        let client = DockerClient::new(target)?;
        let engine = Arc::new(DockerEngine::new(client)) as Arc<dyn Engine>;

        let mut app = Self {
            engine,
            current_tab: Tab::Containers,
            // Containers
            containers: Vec::new(),
            selected_container: 0,
            show_details: false,
            container_detail: None,
            // Images
            images: Vec::new(),
            selected_image: 0,
            show_image_details: false,
            image_detail: None,
            // Volumes
            volumes: Vec::new(),
            selected_volume: 0,
            show_volume_details: false,
            volume_detail: None,
            // Networks
            networks: Vec::new(),
            selected_network: 0,
            show_network_details: false,
            network_detail: None,
            // Global
            should_quit: false,
            modal: None,
            modal_selected: false,
        };

        // Load initial data
        app.refresh_containers().await?;

        Ok(app)
    }

    pub async fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        // If modal is open, handle modal navigation
        if self.modal.is_some() {
            match key.code {
                KeyCode::Left | KeyCode::Char('h') => {
                    self.modal_selected = false;
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    self.modal_selected = true;
                }
                KeyCode::Tab => {
                    self.modal_selected = !self.modal_selected;
                }
                KeyCode::Enter | KeyCode::Char('y') if self.modal_selected => {
                    self.confirm_modal_action().await?;
                }
                KeyCode::Esc | KeyCode::Char('n') | KeyCode::Char('q') => {
                    self.modal = None;
                }
                _ => {}
            }
            return Ok(());
        }

        // Normal key handling when no modal is open
        match key.code {
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            KeyCode::Tab => {
                self.current_tab = self.current_tab.next();
                self.refresh_current_tab().await?;
            }
            KeyCode::BackTab => {
                self.current_tab = self.current_tab.prev();
                self.refresh_current_tab().await?;
            }
            KeyCode::Char('1') => {
                self.current_tab = Tab::Containers;
                self.refresh_current_tab().await?;
            }
            KeyCode::Char('2') => {
                self.current_tab = Tab::Images;
                self.refresh_current_tab().await?;
            }
            KeyCode::Char('3') => {
                self.current_tab = Tab::Volumes;
                self.refresh_current_tab().await?;
            }
            KeyCode::Char('4') => {
                self.current_tab = Tab::Networks;
                self.refresh_current_tab().await?;
            }
            KeyCode::Char('5') => {
                self.current_tab = Tab::System;
                self.refresh_current_tab().await?;
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                self.refresh_current_tab().await?;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.select_next();
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.select_prev();
            }
            KeyCode::Char('s') => {
                if self.current_tab == Tab::Containers {
                    self.start_selected_container().await?;
                }
            }
            KeyCode::Char('t') => {
                if self.current_tab == Tab::Containers {
                    self.stop_selected_container().await?;
                }
            }
            KeyCode::Char('d') => {
                self.show_delete_confirmation();
            }
            KeyCode::Char('x') => {
                if self.current_tab == Tab::Containers {
                    self.restart_selected_container().await?;
                }
            }
            KeyCode::Enter | KeyCode::Char('i') => {
                self.toggle_details().await?;
            }
            _ => {}
        }
        Ok(())
    }

    pub async fn tick(&mut self) -> Result<()> {
        // Periodic updates
        Ok(())
    }

    async fn refresh_current_tab(&mut self) -> Result<()> {
        match self.current_tab {
            Tab::Containers => self.refresh_containers().await?,
            Tab::Images => self.refresh_images().await?,
            Tab::Volumes => self.refresh_volumes().await?,
            Tab::Networks => self.refresh_networks().await?,
            Tab::System => {}
        }
        Ok(())
    }

    async fn refresh_containers(&mut self) -> Result<()> {
        let containers = self.engine.list_containers(ContainerFilter::default()).await?;
        self.containers = containers;

        // Reset selection if out of bounds
        if self.selected_container >= self.containers.len() && !self.containers.is_empty() {
            self.selected_container = self.containers.len() - 1;
        }

        Ok(())
    }

    async fn refresh_images(&mut self) -> Result<()> {
        let images = self.engine.list_images(ImageFilter::default()).await?;
        self.images = images;

        if self.selected_image >= self.images.len() && !self.images.is_empty() {
            self.selected_image = self.images.len() - 1;
        }

        Ok(())
    }

    async fn refresh_volumes(&mut self) -> Result<()> {
        let volumes = self.engine.list_volumes(VolumeFilter::default()).await?;
        self.volumes = volumes;

        if self.selected_volume >= self.volumes.len() && !self.volumes.is_empty() {
            self.selected_volume = self.volumes.len() - 1;
        }

        Ok(())
    }

    async fn refresh_networks(&mut self) -> Result<()> {
        let networks = self.engine.list_networks(NetworkFilter::default()).await?;
        self.networks = networks;

        if self.selected_network >= self.networks.len() && !self.networks.is_empty() {
            self.selected_network = self.networks.len() - 1;
        }

        Ok(())
    }

    fn select_next(&mut self) {
        match self.current_tab {
            Tab::Containers => {
                if !self.containers.is_empty() {
                    self.selected_container = (self.selected_container + 1) % self.containers.len();
                }
            }
            Tab::Images => {
                if !self.images.is_empty() {
                    self.selected_image = (self.selected_image + 1) % self.images.len();
                }
            }
            Tab::Volumes => {
                if !self.volumes.is_empty() {
                    self.selected_volume = (self.selected_volume + 1) % self.volumes.len();
                }
            }
            Tab::Networks => {
                if !self.networks.is_empty() {
                    self.selected_network = (self.selected_network + 1) % self.networks.len();
                }
            }
            Tab::System => {}
        }
    }

    fn select_prev(&mut self) {
        match self.current_tab {
            Tab::Containers => {
                if !self.containers.is_empty() {
                    if self.selected_container == 0 {
                        self.selected_container = self.containers.len() - 1;
                    } else {
                        self.selected_container -= 1;
                    }
                }
            }
            Tab::Images => {
                if !self.images.is_empty() {
                    if self.selected_image == 0 {
                        self.selected_image = self.images.len() - 1;
                    } else {
                        self.selected_image -= 1;
                    }
                }
            }
            Tab::Volumes => {
                if !self.volumes.is_empty() {
                    if self.selected_volume == 0 {
                        self.selected_volume = self.volumes.len() - 1;
                    } else {
                        self.selected_volume -= 1;
                    }
                }
            }
            Tab::Networks => {
                if !self.networks.is_empty() {
                    if self.selected_network == 0 {
                        self.selected_network = self.networks.len() - 1;
                    } else {
                        self.selected_network -= 1;
                    }
                }
            }
            Tab::System => {}
        }
    }

    async fn start_selected_container(&mut self) -> Result<()> {
        if let Some(container) = self.containers.get(self.selected_container) {
            self.engine.start_container(&container.id).await?;
            self.refresh_containers().await?;
        }
        Ok(())
    }

    async fn stop_selected_container(&mut self) -> Result<()> {
        if let Some(container) = self.containers.get(self.selected_container) {
            self.engine.stop_container(&container.id, None).await?;
            self.refresh_containers().await?;
        }
        Ok(())
    }

    async fn restart_selected_container(&mut self) -> Result<()> {
        if let Some(container) = self.containers.get(self.selected_container) {
            self.engine.restart_container(&container.id, None).await?;
            self.refresh_containers().await?;
        }
        Ok(())
    }

    fn show_delete_confirmation(&mut self) {
        match self.current_tab {
            Tab::Containers => {
                if let Some(container) = self.containers.get(self.selected_container) {
                    let dialog = ConfirmDialog::new(
                        "Delete Container",
                        format!(
                            "Are you sure you want to delete container '{}'?\n\nThis action cannot be undone.",
                            container.name
                        ),
                    )
                    .dangerous()
                    .with_labels("Delete", "Cancel");

                    self.modal = Some((dialog, ModalAction::DeleteContainer(container.id.clone())));
                    self.modal_selected = false;
                }
            }
            Tab::Images => {
                if let Some(image) = self.images.get(self.selected_image) {
                    let name = image.repo_tags.first().unwrap_or(&image.id.0);
                    let dialog = ConfirmDialog::new(
                        "Delete Image",
                        format!(
                            "Are you sure you want to delete image '{}'?\n\nThis action cannot be undone.",
                            name
                        ),
                    )
                    .dangerous()
                    .with_labels("Delete", "Cancel");

                    self.modal = Some((dialog, ModalAction::DeleteImage(image.id.clone())));
                    self.modal_selected = false;
                }
            }
            Tab::Volumes => {
                if let Some(volume) = self.volumes.get(self.selected_volume) {
                    let dialog = ConfirmDialog::new(
                        "Delete Volume",
                        format!(
                            "Are you sure you want to delete volume '{}'?\n\nThis action cannot be undone.",
                            volume.name.0
                        ),
                    )
                    .dangerous()
                    .with_labels("Delete", "Cancel");

                    self.modal = Some((dialog, ModalAction::DeleteVolume(volume.name.clone())));
                    self.modal_selected = false;
                }
            }
            Tab::Networks => {
                if let Some(network) = self.networks.get(self.selected_network) {
                    let dialog = ConfirmDialog::new(
                        "Delete Network",
                        format!(
                            "Are you sure you want to delete network '{}'?\n\nThis action cannot be undone.",
                            network.name
                        ),
                    )
                    .dangerous()
                    .with_labels("Delete", "Cancel");

                    self.modal = Some((dialog, ModalAction::DeleteNetwork(network.id.clone())));
                    self.modal_selected = false;
                }
            }
            Tab::System => {}
        }
    }

    async fn confirm_modal_action(&mut self) -> Result<()> {
        if let Some((_, action)) = self.modal.take() {
            match action {
                ModalAction::DeleteContainer(id) => {
                    self.engine
                        .delete_container(&id, tenor_core::DeleteContainerOpts::default())
                        .await?;
                    self.refresh_containers().await?;
                }
                ModalAction::DeleteImage(id) => {
                    self.engine.remove_image(&id, false).await?;
                    self.refresh_images().await?;
                }
                ModalAction::DeleteVolume(id) => {
                    self.engine.remove_volume(&id, false).await?;
                    self.refresh_volumes().await?;
                }
                ModalAction::DeleteNetwork(id) => {
                    self.engine.remove_network(&id).await?;
                    self.refresh_networks().await?;
                }
            }
        }
        Ok(())
    }

    pub fn get_modal(&self) -> Option<&(ConfirmDialog, ModalAction)> {
        self.modal.as_ref()
    }

    pub fn is_modal_confirm_selected(&self) -> bool {
        self.modal_selected
    }

    async fn toggle_details(&mut self) -> Result<()> {
        match self.current_tab {
            Tab::Containers => {
                if self.show_details {
                    self.show_details = false;
                    self.container_detail = None;
                } else if let Some(container) = self.containers.get(self.selected_container) {
                    let detail = self.engine.inspect_container(&container.id).await?;
                    self.container_detail = Some(detail);
                    self.show_details = true;
                }
            }
            Tab::Images => {
                if self.show_image_details {
                    self.show_image_details = false;
                    self.image_detail = None;
                } else if let Some(image) = self.images.get(self.selected_image) {
                    let detail = self.engine.inspect_image(&image.id).await?;
                    self.image_detail = Some(detail);
                    self.show_image_details = true;
                }
            }
            Tab::Volumes => {
                if self.show_volume_details {
                    self.show_volume_details = false;
                    self.volume_detail = None;
                } else if let Some(volume) = self.volumes.get(self.selected_volume) {
                    let detail = self.engine.inspect_volume(&volume.name).await?;
                    self.volume_detail = Some(detail);
                    self.show_volume_details = true;
                }
            }
            Tab::Networks => {
                if self.show_network_details {
                    self.show_network_details = false;
                    self.network_detail = None;
                } else if let Some(network) = self.networks.get(self.selected_network) {
                    let detail = self.engine.inspect_network(&network.id).await?;
                    self.network_detail = Some(detail);
                    self.show_network_details = true;
                }
            }
            Tab::System => {}
        }
        Ok(())
    }
}
