mod app;
mod components;
mod event;
mod screens;
mod terminal;
mod ui;

use anyhow::Result;
use app::App;
use event::{Event, EventHandler};
use terminal::setup_terminal;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup terminal
    let mut terminal = setup_terminal()?;

    // Create app
    let mut app = App::new().await?;

    // Create event handler
    let mut events = EventHandler::new();

    // Main loop
    loop {
        // Render
        terminal.draw(|frame| {
            ui::render(&mut app, frame);
        })?;

        // Handle events
        if let Some(event) = events.next().await {
            match event {
                Event::Quit => break,
                Event::Key(key) => {
                    app.handle_key(key).await?;
                }
                Event::Tick => {
                    app.tick().await?;
                }
                Event::Resize(_, _) => {}
            }
        }
    }

    // Restore terminal
    terminal::restore_terminal()?;

    Ok(())
}
