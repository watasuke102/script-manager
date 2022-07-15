use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::process::Process;

pub fn draw_process_log<B: Backend>(
    f: &mut Frame<B>,
    process_list: &Vec<Process>,
    focused_index: usize,
) {
    if process_list.len() == 0 {
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
            vec![Constraint::Ratio(1, process_list.len() as u32); process_list.len()].as_ref(),
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

    for (i, process) in process_list.into_iter().enumerate() {
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
        chunks[focused_index].x + (process_list[focused_index].filter.len() as u16) + 1,
        chunks[focused_index].y + 2,
    );
}
