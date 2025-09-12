use std::{error::Error, process::Command};

use crate::scripts_find::script_path;

pub fn minimal_dhcp_server_runner(trimmed_line: &str) -> Result<(), Box<dyn Error>> {
    // Split and collect all whitespace-separated tokens
    let args: Vec<&str> = trimmed_line.split_whitespace().collect();

    // 1. Validate we have the token token: [start or stop]
    if args.len() < 1 {
        return Err(
            "Incorrect amount of arguments: Usage: minimal_dchp_server <offered_ip>".into(),
        );
    }

    // resolve the .py location
    let script = script_path("minimal_dhcp_server.py")?;
    // Give sensor time to carry out the DHCP operation.
    // Also used by the server to terminate in the event we don't
    let runtime = 20.to_string();
    let offered_ip = args[1];
    let status = Command::new("python")
        .arg(script)
        .arg(runtime)
        .arg(offered_ip)
        .status()?;

    if !status.success() {
        return Err("Minimal DHCP server failed. ".into());
    }
    return Ok(());
}
