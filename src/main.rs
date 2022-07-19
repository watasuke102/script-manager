use app::App;
use crossterm::{
  event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, time::Duration};
use tui::{backend::CrosstermBackend, Terminal};

mod process;
mod terminal;
use terminal::draw_process_log;
mod app;

fn main() -> Result<(), Box<dyn Error>> {
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  // main loop
  let mut app = App::new();
  loop {
    terminal.draw(|f| draw_process_log(f, &mut app))?;

    if poll(Duration::from_millis(100))? {
      if let Event::Key(key) = event::read()? {
        // action
        if key.modifiers.contains(KeyModifiers::CONTROL) {
          match key.code {
            KeyCode::Char('q') => break,
            KeyCode::Char('w') => break,
            KeyCode::Char('u') => app.clear_current_filter(),
            KeyCode::Char('a') => app.create_process(),
            _ => (),
          }
        } else {
          // edit or move focus
          match key.code {
            // edit filter
            KeyCode::Char(c) => app.push_current_filter(c),
            KeyCode::Backspace => app.pop_current_filter(),
            // move focus
            KeyCode::Tab => app.focus_next(),
            KeyCode::BackTab => app.focus_prev(),
            _ => (),
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
