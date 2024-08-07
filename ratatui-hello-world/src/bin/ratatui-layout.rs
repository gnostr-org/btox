use std::io::{self, stdout};

use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    widgets::Block,
    Frame, Terminal,
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;
    while !should_quit {
        terminal.draw(ui)?;
        should_quit = handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_events() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
fn ui(frame: &mut Frame) {
    let [title_area, main_area, status_area] = ratatui::prelude::Layout::vertical([
        ratatui::prelude::Constraint::Length(1),
        ratatui::prelude::Constraint::Min(0),
        ratatui::prelude::Constraint::Length(1),
    ])
    .areas(frame.area());
    let [left_area, right_area] = ratatui::prelude::Layout::horizontal([
        ratatui::prelude::Constraint::Percentage(50),
        ratatui::prelude::Constraint::Percentage(50),
    ])
    .areas(main_area);

    frame.render_widget(Block::bordered().title("Title Bar"), title_area);
    frame.render_widget(Block::bordered().title("Status Bar"), status_area);
    frame.render_widget(Block::bordered().title("Left"), left_area);
    frame.render_widget(Block::bordered().title("Right"), right_area);
}
