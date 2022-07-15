use crossterm::{
    event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
mod process;
use process::{create_process, Process};
mod terminal;
use std::{error::Error, io, time::Duration};
use terminal::draw_process_log;
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // main loop
    let mut process_list = Vec::<Process>::new();
    let mut focused_index: usize = 0;
    loop {
        terminal.draw(|f| draw_process_log(f, &process_list, focused_index))?;

        if poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                // action
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('w') => break,
                        KeyCode::Char('u') => {
                            if process_list.len() != 0 {
                                process_list[focused_index].filter.clear();
                            }
                        }
                        KeyCode::Char('a') => {
                            process_list.push(create_process(&String::from("script/seq.sh")))
                        }
                        _ => (),
                    }
                } else {
                    // edit or move focus
                    match key.code {
                        // edit filter
                        KeyCode::Char(c) => {
                            if process_list.len() != 0 {
                                process_list[focused_index].filter.push(c);
                            }
                        }
                        KeyCode::Backspace => {
                            if process_list.len() != 0 {
                                process_list[focused_index].filter.pop();
                            }
                        }
                        // change focus
                        KeyCode::Tab => {
                            focused_index = if focused_index == process_list.len() - 1 {
                                0
                            } else {
                                focused_index + 1
                            };
                        }
                        KeyCode::BackTab => {
                            focused_index = if focused_index == 0 {
                                process_list.len() - 1
                            } else {
                                focused_index - 1
                            };
                        }
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
