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
    thread: JoinHandle<()>,
}

pub fn create_process(name: &String) -> Process {
    let output = Arc::new(Mutex::new(String::new()));
    let name = Arc::new(name.clone());
    Process {
        name: name.clone(),
        filter: String::new(),
        scroll_x: 0,
        scroll_y: 0,
        output: output.clone(),
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

            loop {
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
