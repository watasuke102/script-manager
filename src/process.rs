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
    pub output: Arc<Mutex<String>>,
    thread: JoinHandle<()>,
}

pub fn create_process(name: &String) -> Process {
    let output = Arc::new(Mutex::new(String::new()));
    let name = Arc::new(name.clone());
    Process {
        name: name.clone(),
        filter: String::new(),
        output: output.clone(),
        thread: thread::spawn(move || {
            let output = output.clone();
            let name = name.clone();
            let mut child = Command::new("/usr/bin/bash")
                .args([name.as_str()])
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();
            let out = child.stdout.as_mut().unwrap();
            let mut buf = [0u8; 1];
            loop {
                out.read(&mut buf).unwrap();
                output.lock().unwrap().push(buf[0] as char);
            }
        }),
    }
}
