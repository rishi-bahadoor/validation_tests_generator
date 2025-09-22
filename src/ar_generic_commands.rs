use std::error::Error;

use crate::dhcp_server::dhcp_server_runner;
use crate::misc::{press_enter, wait_s};

pub fn generic_runner(trimmed_line: &str) -> Result<(), Box<dyn Error>> {
    let parts = trimmed_line.split_whitespace();
    let args: Vec<&str> = parts.collect();

    if args[0] == "wait_s" {
        let timeout = args[1].parse().unwrap();
        wait_s(timeout);
    } else if args[0] == "wait_e" {
        press_enter();
    } else if args[0] == "dhcp_server" {
        dhcp_server_runner()?;
    }
    Ok(())
}
