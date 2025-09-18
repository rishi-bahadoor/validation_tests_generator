use std::{error::Error, process::Command};

use crate::scripts_find::script_path;

pub fn dhcp_server_runner(trimmed_line: &str) -> Result<(), Box<dyn Error>> {
    // Split and collect all whitespace-separated tokens
    let args: Vec<&str> = trimmed_line.split_whitespace().collect();

    // Validate we have the token token: [start or stop]
    if args.len() < 1 {
        return Err(
            "Incorrect amount of arguments: Usage: minimal_dchp_server <offered_ip>".into(),
        );
    }

    // Resolve the .py location
    let script = script_path("minimal_dhcp_server.py")?;

    // Specifies how long to run the server for.
    // 20 seconds was chosen to give sensor time to carry out the DHCP operation.
    // It is also used by the server to self terminate in the event we don't.
    let runtime = 20.to_string();
    let offered_ip = args[0];
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
