use std::error::Error;

use crate::misc::{press_enter, wait_s};

/// Trims, replaces "ccc" with the actual exe path, runs the command,
/// and returns Ok(()) if the process exits successfully or Err on failure.
pub fn generic_runner(line: &str) -> Result<(), Box<dyn Error>> {
    // 1. Prepare command string
    let trimmed = line.trim();
    let parts = trimmed.split_whitespace();
    let args: Vec<&str> = parts.collect();

    if args[0] == "wait_s" {
        let timeout = args[1].parse().unwrap();
        wait_s(timeout);
    } else if args[0] == "wait_e" {
        press_enter();
    }

    Ok(())
}
