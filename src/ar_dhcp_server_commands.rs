use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::{
    error::Error,
    process::{Child, Command},
};

use crate::scripts_find::script_path;

static GLOBAL_CHILD: Lazy<Mutex<Option<Child>>> = Lazy::new(|| Mutex::new(None));

pub fn minimal_dhcp_server_runner(trimmed_line: &str) -> Result<(), Box<dyn Error>> {
    // Split and collect all whitespace-separated tokens
    let args: Vec<&str> = trimmed_line.split_whitespace().collect();

    // 1. Validate we have the token token: [start or stop]
    if args.len() < 1 {
        return Err(
            "Incorrect amount of arguments: Usage: minimal_dchp_server <start/stop>".into(),
        );
    }

    let cmd = args[1];

    if cmd != "start" && cmd != "stop" {
        return Err("Usage: minimal_dchp_server <start/stop>".into());
    }

    if cmd == "start" {
        return start_server();
    }

    if cmd == "stop" {
        return stop_server();
    }

    return Err("Unknown error.".into());
}

fn start_server() -> Result<(), Box<dyn Error>> {
    let mut lock = GLOBAL_CHILD.lock().unwrap();

    if lock.is_some() {
        println!("Python process already running, not starting another.");
        return Ok(());
    }

    // resolve the .py location
    let script = script_path("minimal_dhcp_server.py")?;
    let runtime = 3600.to_string(); // Seconds to run server in the even this program crashes and we don't kill it
    let child = Command::new("python").arg(script).arg(runtime).spawn()?;
    println!("Server process ID: {}", child.id());
    *lock = Some(child);
    return Ok(());
}

fn stop_server() -> Result<(), Box<dyn Error>> {
    let mut lock = GLOBAL_CHILD.lock().unwrap();

    if let Some(mut child) = lock.take() {
        let _ = child.kill(); // ignore error if already exited
        println!("Python process killed.");
    } else {
        println!("No Python process running.");
    }
    return Ok(());
}
