use std::{error::Error, process::Command, thread, time::Duration};

use crate::scripts_find::script_path;

pub fn minimal_dhcp_server_runner() -> Result<(), Box<dyn Error>> {
    // resolve the .py location
    let script = script_path("minimal_dhcp_server.py")?;

    let status = Command::new("python").arg(script).status()?;

    if !status.success() {
        return Err("Failed to run minimal DHCP server.".into());
    }

    println!("Successfully ran minimal DHCP server.");

    return Ok(());
}
