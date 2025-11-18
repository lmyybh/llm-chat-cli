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
mod llm;

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
    let mut app = App::new();

    // Run the app
    let result = run_app(&mut terminal, &mut app).await;

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
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|frame| ui::render(app, frame))?;

        if app.llm_receiver.is_some() {
            // 安全地 take 出 receiver（app.llm_receiver 变为 None）
            let receiver = app.llm_receiver.take().unwrap();

            let mut should_put_back = true;
            let mut done = false;

            // 非阻塞地读取所有可用 token
            while let Ok(token) = receiver.try_recv() {
                if token == "__DONE__" {
                    done = true;
                    should_put_back = false;
                    break;
                } else if token == "__ERROR__" {
                    // 追加错误消息
                    done = true;
                    should_put_back = false;
                    break;
                } else {
                    // 追加 token 到最新 Assistant 消息
                    app.add_streaming_content(token);
                    // 自动滚动到底部（如果用户没手动滚动）
                    app.chat_scroll_offset = 0;
                }
            }

            if done {
                app.is_waiting_for_response = false;
            }

            // 如果还没结束，把 receiver 放回去
            if should_put_back {
                app.llm_receiver = Some(receiver);
            }
        }

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