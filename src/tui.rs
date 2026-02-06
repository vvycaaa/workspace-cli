use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};
use std::io::{self, Stdout};

pub fn run_tui(items: Vec<(String, String)>) -> Result<Option<String>> {
    // Check if there are any workspaces
    if items.is_empty() {
        return Ok(None);
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app logic
    let res = run_app(&mut terminal, items);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    res
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, items: Vec<(String, String)>) -> Result<Option<String>> {
    let mut state = ListState::default();
    state.select(Some(0));

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(f.size());

            let list_items: Vec<ListItem> = items
                .iter()
                .map(|(display, _)| {
                    let lines = vec![Line::from(display.clone())];
                    ListItem::new(lines).style(Style::default().fg(Color::White))
                })
                .collect();

            let list = List::new(list_items)
                .block(Block::default().borders(Borders::ALL).title("Select Workspace"))
                .highlight_style(
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, chunks[0], &mut state);

            let help_text = Paragraph::new(Line::from(vec![
                Span::raw("Use "),
                Span::styled("↑/↓", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to move, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to select, "),
                Span::styled("q/Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to quit"),
            ]))
            .block(Block::default().borders(Borders::ALL));
            
            f.render_widget(help_text, chunks[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(None),
                        KeyCode::Enter => {
                            if let Some(i) = state.selected() {
                                if i < items.len() {
                                    return Ok(Some(items[i].1.clone()));
                                }
                            }
                        }
                        KeyCode::Down => {
                            let i = match state.selected() {
                                Some(i) => {
                                    if i >= items.len() - 1 {
                                        0
                                    } else {
                                        i + 1
                                    }
                                }
                                None => 0,
                            };
                            state.select(Some(i));
                        }
                        KeyCode::Up => {
                            let i = match state.selected() {
                                Some(i) => {
                                    if i == 0 {
                                        items.len() - 1
                                    } else {
                                        i - 1
                                    }
                                }
                                None => 0,
                            };
                            state.select(Some(i));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
