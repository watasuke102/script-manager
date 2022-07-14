use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    text::{Spans, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::process::Process;

pub fn draw_process_log<B: Backend>(
    f: &mut Frame<B>,
    process_list: &Vec<Process>,
    focused_index: i32,
) {
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
        let block = Block::default().title("filter").borders(Borders::ALL);
        f.render_widget(block, chunk[1]);
        // output
        let block = Paragraph::new(Text::from(Spans::from(
            process.output.lock().unwrap().clone(),
        )))
        .wrap(Wrap { trim: true })
        .block(Block::default().title("output").borders(Borders::ALL));
        f.render_widget(block, chunk[2]);
    }
}
