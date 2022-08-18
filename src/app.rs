use std::fs;

use tui::widgets::ListState;

use crate::process::{create_process, Process};

pub enum AppStatus {
  Monitor,
  FileList,
}

pub struct FileList {
  pub names: Option<Vec<String>>,
  pub state: ListState,
  pub selected_index: usize,
}

pub struct App {
  pub process_list: Vec<Process>,
  pub focused_index: usize,
  pub current_status: AppStatus,

  pub file_list: FileList,
}

impl FileList {
  pub fn new() -> FileList {
    FileList {
      names: None,
      state: ListState::default(),
      selected_index: 0,
    }
  }
  pub fn next(&mut self) {
    if let Some(names) = &self.names {
      self.selected_index = if self.selected_index == names.len() - 1 {
        0
      } else {
        self.selected_index + 1
      };
      self.state.select(Some(self.selected_index));
    }
  }
  pub fn prev(&mut self) {
    if let Some(names) = &self.names {
      self.selected_index = if self.selected_index == 0 {
        names.len() - 1
      } else {
        self.selected_index - 1
      };
      self.state.select(Some(self.selected_index));
    }
  }
  pub fn selected_file_name(&self) -> Option<String> {
    if let Some(names) = &self.names {
      return Some(names[self.selected_index].clone());
    } else {
      None
    }
  }
}

impl App {
  pub fn new() -> App {
    App {
      process_list: Vec::new(),
      focused_index: 0,
      current_status: AppStatus::Monitor,

      file_list: FileList::new(),
    }
  }
  fn process_exist(&self) -> bool {
    self.process_list.len() != 0
  }
  pub fn create_process(&mut self, name: &String) {
    self.process_list.push(create_process(name));
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

  pub fn open_file_list(&mut self) {
    self.file_list.names = if let Ok(list) = fs::read_dir("script") {
      Some(
        list
          .into_iter()
          .map(|entry| entry.unwrap().file_name().into_string().unwrap())
          .collect(),
      )
    } else {
      None
    };
    self.file_list.state.select(Some(0));
    self.file_list.selected_index = 0;
    self.current_status = AppStatus::FileList;
  }
}
