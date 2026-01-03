use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::sync::Arc;
use tenor_core::{Container, ContainerFilter, Engine};
use tenor_docker::{ConnectionTarget, DockerClient, DockerEngine};

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

pub struct App {
    pub engine: Arc<dyn Engine>,
    pub current_tab: Tab,
    pub containers: Vec<Container>,
    pub selected_container: usize,
    pub should_quit: bool,
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
            containers: Vec::new(),
            selected_container: 0,
            should_quit: false,
        };

        // Load initial data
        app.refresh_containers().await?;

        Ok(app)
    }

    pub async fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            KeyCode::Tab => {
                self.current_tab = self.current_tab.next();
            }
            KeyCode::BackTab => {
                self.current_tab = self.current_tab.prev();
            }
            KeyCode::Char('1') => self.current_tab = Tab::Containers,
            KeyCode::Char('2') => self.current_tab = Tab::Images,
            KeyCode::Char('3') => self.current_tab = Tab::Volumes,
            KeyCode::Char('4') => self.current_tab = Tab::Networks,
            KeyCode::Char('5') => self.current_tab = Tab::System,
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
                self.start_selected_container().await?;
            }
            KeyCode::Char('t') => {
                self.stop_selected_container().await?;
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
            Tab::Images => {}
            Tab::Volumes => {}
            Tab::Networks => {}
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

    fn select_next(&mut self) {
        if !self.containers.is_empty() {
            self.selected_container = (self.selected_container + 1) % self.containers.len();
        }
    }

    fn select_prev(&mut self) {
        if !self.containers.is_empty() {
            if self.selected_container == 0 {
                self.selected_container = self.containers.len() - 1;
            } else {
                self.selected_container -= 1;
            }
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
}
