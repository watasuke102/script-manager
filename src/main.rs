use app::{App, AppStatus};
use crossterm::{
  event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dotenv::dotenv;
use std::{error::Error, io, time::Duration};
use tui::{backend::CrosstermBackend, Terminal};

mod process;
mod terminal;
use terminal::{draw_file_list, draw_process_log};
mod app;

fn main() -> Result<(), Box<dyn Error>> {
  dotenv().ok();
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  // main loop
  let mut app = App::new();
  loop {
    terminal.draw(|f| {
      draw_process_log(f, &mut app);
      match app.current_status {
        AppStatus::FileList => draw_file_list(f, &mut app.file_list),
        _ => (),
      }
    })?;

    if poll(Duration::from_millis(100))? {
      if let Event::Key(key) = event::read()? {
        // action
        if key.modifiers.contains(KeyModifiers::CONTROL) {
          match key.code {
            KeyCode::Char('q') => break,
            KeyCode::Char('w') => break,
            KeyCode::Char('x') => app.kill_current_process(),
            KeyCode::Char('u') => app.clear_current_filter(),
            KeyCode::Char('a') => app.open_file_list(),
            _ => (),
          }
        } else {
          // edit or move focus
          match app.current_status {
            AppStatus::Monitor => match key.code {
              // edit filter
              KeyCode::Char(c) => app.push_current_filter(c),
              KeyCode::Backspace => app.pop_current_filter(),
              // move focus
              KeyCode::Tab => app.focus_next(),
              KeyCode::BackTab => app.focus_prev(),
              _ => (),
            },
            AppStatus::FileList => match key.code {
              KeyCode::Char('q') => app.current_status = AppStatus::Monitor,
              KeyCode::Up => app.file_list.prev(),
              KeyCode::Down => app.file_list.next(),
              KeyCode::Enter => {
                if let Some(name) = app.file_list.selected_file_name() {
                  app.create_process(&format!("script/{}", name));
                  app.current_status = AppStatus::Monitor;
                }
              }
              _ => (),
            },
          }
        }
      }
    }
  }

  disable_raw_mode()?;
  execute!(
    terminal.backend_mut(),
    LeaveAlternateScreen,
    DisableMouseCapture
  )?;
  terminal.show_cursor()?;

  Ok(())
}
