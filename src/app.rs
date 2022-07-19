use crate::process::{create_process, Process};

pub struct App {
  pub process_list:  Vec<Process>,
  pub focused_index: usize,
}

impl App {
  pub fn new() -> App {
    App {
      process_list:  Vec::new(),
      focused_index: 0,
    }
  }
  fn process_exist(&self) -> bool {
    self.process_list.len() != 0
  }
  pub fn create_process(&mut self) {
    self
      .process_list
      .push(create_process(&String::from("script/seq.sh")));
  }
  // edit filter
  pub fn push_current_filter(&mut self, c: char) {
    if self.process_exist() {
      self.process_list[self.focused_index].filter.push(c);
    }
  }
  pub fn pop_current_filter(&mut self) {
    if self.process_exist() {
      self.process_list[self.focused_index].filter.pop();
    }
  }
  pub fn clear_current_filter(&mut self) {
    if self.process_exist() {
      self.process_list[self.focused_index].filter.clear();
    }
  }
  // move focus
  pub fn focus_next(&mut self) {
    self.focused_index = if self.focused_index == self.process_list.len() - 1 {
      0
    } else {
      self.focused_index + 1
    };
  }
  pub fn focus_prev(&mut self) {
    self.focused_index = if self.focused_index == 0 {
      self.process_list.len() - 1
    } else {
      self.focused_index - 1
    };
  }
}
