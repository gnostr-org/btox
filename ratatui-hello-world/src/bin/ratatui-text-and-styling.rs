use std::io::{self, stdout};
use ratatui::prelude::Stylize;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
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
    let areas = ratatui::prelude::Layout::vertical([ratatui::prelude::Constraint::Length(1); 4]).split(frame.area());

    let line = ratatui::text::Line::from(vec![
        ratatui::text::Span::raw("Hello "),
        ratatui::text::Span::styled(
            "World",
            ratatui::style::Style::new()
                .fg(ratatui::style::Color::Green)
                .bg(ratatui::style::Color::White)
                .add_modifier(ratatui::style::Modifier::BOLD),
        ),
        "!".red().on_light_yellow().italic(),
    ]);
    frame.render_widget(line, areas[0]);

    // using the short-hand syntax and implicit conversions
    let paragraph = ratatui::widgets::Paragraph::new("Hello World!".red().on_white().bold());
    frame.render_widget(paragraph, areas[1]);

    // style the whole widget instead of just the text
    let paragraph = ratatui::widgets::Paragraph::new("Hello World!").style(ratatui::style::Style::new().red().on_white());
    frame.render_widget(paragraph, areas[2]);

    // use the simpler short-hand syntax
    let paragraph = ratatui::widgets::Paragraph::new("Hello World!").blue().on_yellow();
    frame.render_widget(paragraph, areas[3]);
}
