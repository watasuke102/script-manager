use tui::{
  backend::Backend,
  layout::{Constraint, Direction, Layout, Rect},
  text::{Span, Spans, Text},
  widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
  Frame,
};

use crate::{app::FileList, App};

fn popup(r: Rect, margin_x: u16, margin_y: u16) -> Rect {
  Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage(margin_x),
      Constraint::Min(1),
      Constraint::Percentage(margin_x),
    ])
    .split(
      Layout::default()
        .direction(Direction::Vertical)
        .constraints([
          Constraint::Percentage(margin_y + 2),
          Constraint::Min(1),
          Constraint::Percentage(margin_y),
        ])
        .split(r)[1],
    )[1]
}

pub fn draw_process_log<B: Backend>(f: &mut Frame<B>, app: &mut App) {
  if app.process_list.len() == 0 {
    let chunk = Layout::default()
      .constraints([Constraint::Percentage(100)])
      .split(f.size());
    let block = Paragraph::new(Text::from(Spans::from(String::from(
      "There are no process",
    ))));
    f.render_widget(block, chunk[0]);
    return;
  }

  let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints(
      vec![Constraint::Ratio(1, app.process_list.len() as u32); app.process_list.len()].as_ref(),
    )
    .split(f.size());

  let chunk_template = Layout::default()
    .direction(Direction::Vertical)
    .constraints(
      [
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Min(1),
      ]
      .as_ref(),
    );

  for (i, process) in app.process_list.iter().enumerate() {
    let chunk = chunk_template.split(chunks[i]);
    // name
    let block = Paragraph::new(Text::from(Spans::from(process.name.as_ref().clone())));
    f.render_widget(block, chunk[0]);
    // filter
    let block = Paragraph::new(Text::from(Spans::from(process.filter.clone())))
      .block(Block::default().title("filter").borders(Borders::ALL));
    f.render_widget(block, chunk[1]);
    // output
    let output_lines = process.output.lock().unwrap().clone();
    let output_lines: Vec<Spans> = output_lines
      .lines()
      .filter(|s| s.contains(&process.filter))
      .map(|s| Spans::from(Span::raw(s)))
      .collect();
    let output_len = output_lines.len() as u16;
    let block = Paragraph::new(output_lines)
      .wrap(Wrap { trim: true })
      .scroll((process.scroll_x, output_len - process.scroll_y))
      .block(Block::default().title("output").borders(Borders::ALL));
    f.render_widget(block, chunk[2]);
  }

  f.set_cursor(
    chunks[app.focused_index].x + (app.process_list[app.focused_index].filter.len() as u16) + 1,
    chunks[app.focused_index].y + 2,
  );
}

pub fn draw_file_list<B: Backend>(f: &mut Frame<B>, file_list: &mut FileList) {
  if let None = file_list.names {
    let chunk = Layout::default()
      .constraints([Constraint::Percentage(100)])
      .split(f.size());
    let block = Paragraph::new(Text::from(Spans::from(String::from(
      "Cannot open 'script' dir",
    ))));
    f.render_widget(block, chunk[0]);
    return;
  }
  let dir_list = file_list.names.as_ref().unwrap();

  let list: Vec<ListItem> = dir_list
    .iter()
    .map(|e| ListItem::new(Spans::from(String::from(e))))
    .collect();

  let block = List::new(list)
    .block(Block::default().borders(Borders::ALL))
    .highlight_symbol("> ");

  let area = popup(f.size(), 12, 12);
  f.render_widget(Clear, area);
  f.render_stateful_widget(block, area, &mut file_list.state);
}
