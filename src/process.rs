use std::{
  io::Read,
  process::{Command, Stdio},
  sync::{Arc, Mutex},
  thread::{self, JoinHandle},
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Process {
  pub name: Arc<String>,
  pub filter: String,
  pub scroll_x: u16,
  /// describe scrolled row count **from bottom (latest output)**
  pub scroll_y: u16,
  pub output: Arc<Mutex<String>>,
  living: Arc<Mutex<bool>>,
  thread: JoinHandle<()>,
}

impl Process {
  pub fn kill(&mut self) {
    *self.living.lock().unwrap() = false;
  }
}

pub fn create_process(name: &String) -> Process {
  // but I should use AtomicBool in general
  let living = Arc::new(Mutex::new(true));
  let name = Arc::new(name.clone());
  let output = Arc::new(Mutex::new(String::new()));
  Process {
    name: name.clone(),
    filter: String::new(),
    scroll_x: 0,
    scroll_y: 0,
    output: output.clone(),
    living: living.clone(),
    thread: thread::spawn(move || {
      let output = output.clone();
      let name = name.clone();
      let mut child = Command::new("/usr/bin/bash")
        .args([name.as_str()])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
      let mut buf = [0u8; 1];

      while *living.try_lock().unwrap() {
        match child.try_wait() {
          Ok(Some(stat)) => {
            output
              .lock()
              .unwrap()
              .push_str(&format!("\nExited ({stat})\n"));
            return;
          }
          Ok(None) => {
            child.stdout.as_mut().unwrap().read(&mut buf).unwrap();
            output.lock().unwrap().push(buf[0] as char);
          }
          Err(e) => output
            .lock()
            .unwrap()
            .push_str(&format!("\nError occured ({e})\n")),
        }
      }
    }),
  }
}
