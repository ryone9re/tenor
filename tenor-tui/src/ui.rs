use crate::app::{App, Tab};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Wrap},
    Frame,
};

pub fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tabs
            Constraint::Min(0),    // Content
            Constraint::Length(1), // Status bar
        ])
        .split(frame.area());

    render_tabs(app, frame, chunks[0]);
    render_content(app, frame, chunks[1]);
    render_status_bar(app, frame, chunks[2]);

    // Render modal on top if present
    if let Some((dialog, _)) = app.get_modal() {
        dialog.render(frame, app.is_modal_confirm_selected());
    }
}

fn render_tabs(app: &App, frame: &mut Frame, area: Rect) {
    let tabs_list = Tab::all();
    let titles: Vec<Line> = tabs_list
        .iter()
        .map(|t| {
            let style = if *t == app.current_tab {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            Line::from(Span::styled(t.title(), style))
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tenor"))
        .select(match app.current_tab {
            Tab::Containers => 0,
            Tab::Images => 1,
            Tab::Volumes => 2,
            Tab::Networks => 3,
            Tab::System => 4,
        })
        .style(Style::default())
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    frame.render_widget(tabs, area);
}

fn render_content(app: &App, frame: &mut Frame, area: Rect) {
    match app.current_tab {
        Tab::Containers => render_containers(app, frame, area),
        Tab::Images => render_placeholder("Images", frame, area),
        Tab::Volumes => render_placeholder("Volumes", frame, area),
        Tab::Networks => render_placeholder("Networks", frame, area),
        Tab::System => render_placeholder("System", frame, area),
    }
}

fn render_containers(app: &App, frame: &mut Frame, area: Rect) {
    if app.show_details {
        // Split view: list on left, details on right
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(area);

        render_container_list(app, frame, chunks[0]);
        render_container_details(app, frame, chunks[1]);
    } else {
        // Full width list
        render_container_list(app, frame, area);
    }
}

fn render_container_list(app: &App, frame: &mut Frame, area: Rect) {
    let items: Vec<ListItem> = app
        .containers
        .iter()
        .enumerate()
        .map(|(i, container)| {
            let style = if i == app.selected_container {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let state_color = match container.state {
                tenor_core::ContainerState::Running => Color::Green,
                tenor_core::ContainerState::Exited => Color::Red,
                tenor_core::ContainerState::Paused => Color::Yellow,
                tenor_core::ContainerState::Restarting => Color::Cyan,
                tenor_core::ContainerState::Dead => Color::DarkGray,
                tenor_core::ContainerState::Unknown => Color::Gray,
            };

            let content = format!(
                "{:<20} {:<15} {:<30} {}",
                container.name,
                format!("{}", container.state),
                container.image,
                container.status
            );

            ListItem::new(Line::from(vec![
                Span::styled("● ", Style::default().fg(state_color)),
                Span::styled(content, style),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Containers ({})", app.containers.len())),
        )
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    frame.render_widget(list, area);
}

fn render_container_details(app: &App, frame: &mut Frame, area: Rect) {
    if let Some(detail) = &app.container_detail {
        let mut lines = vec![
            Line::from(vec![
                Span::styled("Name: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&detail.name),
            ]),
            Line::from(vec![
                Span::styled("ID: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(detail.id.as_ref()),
            ]),
            Line::from(vec![
                Span::styled("Image: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&detail.image),
            ]),
            Line::from(vec![
                Span::styled("State: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(
                    format!("{}", detail.state),
                    Style::default().fg(match detail.state {
                        tenor_core::ContainerState::Running => Color::Green,
                        tenor_core::ContainerState::Exited => Color::Red,
                        _ => Color::Gray,
                    }),
                ),
            ]),
            Line::from(vec![
                Span::styled("Status: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&detail.status),
            ]),
            Line::from(""),
        ];

        // Command
        if !detail.command.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("Command: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(detail.command.join(" ")),
            ]));
        }

        // Entrypoint
        if !detail.entrypoint.is_empty() {
            lines.push(Line::from(vec![
                Span::styled(
                    "Entrypoint: ",
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(detail.entrypoint.join(" ")),
            ]));
        }

        // Mounts
        if !detail.mounts.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "Mounts:",
                Style::default().add_modifier(Modifier::BOLD),
            )));
            for mount in &detail.mounts {
                lines.push(Line::from(format!(
                    "  {} → {} ({})",
                    mount.source, mount.destination, mount.mode
                )));
            }
        }

        // Networks
        if !detail.network_settings.networks.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "Networks:",
                Style::default().add_modifier(Modifier::BOLD),
            )));
            for network in &detail.network_settings.networks {
                lines.push(Line::from(format!("  {}", network)));
            }
            if let Some(ip) = &detail.network_settings.ip_address {
                lines.push(Line::from(format!("  IP: {}", ip)));
            }
        }

        // Environment
        if !detail.env.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "Environment:",
                Style::default().add_modifier(Modifier::BOLD),
            )));
            for (i, env) in detail.env.iter().enumerate() {
                if i < 10 {
                    // Show first 10
                    lines.push(Line::from(format!("  {}", env)));
                }
            }
            if detail.env.len() > 10 {
                lines.push(Line::from(format!("  ... and {} more", detail.env.len() - 10)));
            }
        }

        // Labels
        if !detail.labels.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "Labels:",
                Style::default().add_modifier(Modifier::BOLD),
            )));
            for (i, (key, value)) in detail.labels.iter().enumerate() {
                if i < 5 {
                    // Show first 5
                    lines.push(Line::from(format!("  {}: {}", key, value)));
                }
            }
            if detail.labels.len() > 5 {
                lines.push(Line::from(format!(
                    "  ... and {} more",
                    detail.labels.len() - 5
                )));
            }
        }

        let paragraph = Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Container Details"),
            )
            .wrap(Wrap { trim: false });

        frame.render_widget(paragraph, area);
    } else {
        let text = Paragraph::new("Loading...")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Container Details"),
            )
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(text, area);
    }
}

fn render_placeholder(title: &str, frame: &mut Frame, area: Rect) {
    let text = Paragraph::new(format!("{} view - Coming soon", title))
        .block(Block::default().borders(Borders::ALL).title(title))
        .style(Style::default().fg(Color::Gray));

    frame.render_widget(text, area);
}

fn render_status_bar(app: &App, frame: &mut Frame, area: Rect) {
    let help_text = if app.get_modal().is_some() {
        "←→/hl: select | Enter/y: confirm | Esc/n/q: cancel"
    } else {
        match app.current_tab {
            Tab::Containers => {
                if app.show_details {
                    "q: quit | r: refresh | ↑↓/jk: navigate | Enter/i: close details | s: start | t: stop | x: restart | d: delete"
                } else {
                    "q: quit | r: refresh | ↑↓/jk: navigate | Enter/i: details | s: start | t: stop | x: restart | d: delete | 1-5: tabs"
                }
            }
            _ => "q: quit | r: refresh | 1-5: switch tabs",
        }
    };

    let status = Paragraph::new(help_text).style(Style::default().fg(Color::DarkGray));

    frame.render_widget(status, area);
}
