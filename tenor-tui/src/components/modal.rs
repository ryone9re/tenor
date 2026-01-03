use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

/// Confirmation dialog state
#[derive(Debug, Clone)]
pub struct ConfirmDialog {
    pub title: String,
    pub message: String,
    pub confirm_label: String,
    pub cancel_label: String,
    pub dangerous: bool,
}

impl ConfirmDialog {
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            confirm_label: "Yes".to_string(),
            cancel_label: "No".to_string(),
            dangerous: false,
        }
    }

    pub fn dangerous(mut self) -> Self {
        self.dangerous = true;
        self
    }

    pub fn with_labels(mut self, confirm: impl Into<String>, cancel: impl Into<String>) -> Self {
        self.confirm_label = confirm.into();
        self.cancel_label = cancel.into();
        self
    }

    pub fn render(&self, f: &mut Frame, selected: bool) {
        let area = centered_rect(60, 30, f.area());

        // Clear the area
        f.render_widget(Clear, area);

        // Draw the dialog box
        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.title.as_str())
            .border_style(if self.dangerous {
                Style::default().fg(Color::Red)
            } else {
                Style::default().fg(Color::Yellow)
            });

        let inner = block.inner(area);
        f.render_widget(block, area);

        // Split into message area and button area
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(3), Constraint::Length(3)])
            .split(inner);

        // Render message
        let message = Paragraph::new(self.message.as_str())
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);
        f.render_widget(message, chunks[0]);

        // Render buttons
        let button_area = chunks[1];
        let button_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ])
            .split(button_area);

        // Cancel button (left)
        let cancel_style = if !selected {
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let cancel_button = Paragraph::new(Line::from(vec![Span::styled(
            format!("[ {} ]", self.cancel_label),
            cancel_style,
        )]))
        .alignment(Alignment::Center);
        f.render_widget(cancel_button, button_chunks[1]);

        // Confirm button (right)
        let confirm_style = if selected {
            Style::default()
                .fg(if self.dangerous {
                    Color::White
                } else {
                    Color::Black
                })
                .bg(if self.dangerous {
                    Color::Red
                } else {
                    Color::Green
                })
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(if self.dangerous {
                Color::Red
            } else {
                Color::Green
            })
        };

        let confirm_button = Paragraph::new(Line::from(vec![Span::styled(
            format!("[ {} ]", self.confirm_label),
            confirm_style,
        )]))
        .alignment(Alignment::Center);
        f.render_widget(confirm_button, button_chunks[2]);
    }
}

/// Helper function to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
