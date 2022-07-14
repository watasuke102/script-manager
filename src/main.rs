use std::{
    io::Read,
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

#[allow(dead_code)]
#[derive(Debug)]
struct Process {
    name: Arc<String>,
    filter: String,
    output: Arc<Mutex<String>>,
    thread: JoinHandle<()>,
}

fn main() {
    let process = create_process(&String::from("script/1.sh"));
    loop {
        println!("{:?}", process);
        thread::sleep(std::time::Duration::from_millis(500));
    }
}

fn create_process(name: &String) -> Process {
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
