use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use std::time::Duration;

pub enum Event {
    Key(KeyEvent),
    Tick,
    #[allow(dead_code)]
    Resize(u16, u16),
    Quit,
}

pub struct EventHandler {
    tick_rate: Duration,
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            tick_rate: Duration::from_millis(250),
        }
    }

    pub async fn next(&mut self) -> Option<Event> {
        if event::poll(self.tick_rate).ok()? {
            match event::read().ok()? {
                CrosstermEvent::Key(key) => {
                    if key.code == event::KeyCode::Char('q') {
                        return Some(Event::Quit);
                    }
                    Some(Event::Key(key))
                }
                CrosstermEvent::Resize(w, h) => Some(Event::Resize(w, h)),
                _ => Some(Event::Tick),
            }
        } else {
            Some(Event::Tick)
        }
    }
}
