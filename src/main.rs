// src/main.rs
use std::io;
use crossterm::{
    event::{self, Event, DisableMouseCapture, EnableMouseCapture, KeyEventKind, MouseEvent, MouseEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{backend::CrosstermBackend, Terminal};

mod app;
mod ui;
mod model;

use app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let app = App::new();

    // Run the app
    let result = run_app(&mut terminal, app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("{err}");
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui::render(&mut app, frame))?;

        if event::poll(std::time::Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        if key.code == crossterm::event::KeyCode::Char('c')
                            && key.modifiers == crossterm::event::KeyModifiers::CONTROL
                        {
                            break;
                        }
                        app.handle_input_key(key);
                    }
                }

                Event::Mouse(MouseEvent {kind, ..}) => {
                    match kind {
                        MouseEventKind::ScrollUp => {
                            app.chat_scroll_offset = app.chat_scroll_offset.saturating_add(3);
                        }
                        MouseEventKind::ScrollDown => {
                            app.chat_scroll_offset = app.chat_scroll_offset.saturating_sub(3);
                        }
                        _ => {}
                    }
                }

                _ => {}
            }
        }
    }
    Ok(())
}