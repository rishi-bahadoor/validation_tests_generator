use std::{error::Error, io, path::Path, process::Command, sync::atomic::AtomicBool};

use crate::misc::{get_key_entry_y, press_enter_no_message};

pub static VERBOSE: AtomicBool = AtomicBool::new(false);

const PATH_CCC_EXE: &str = "./ccc.exe";

/// Trims, replaces "ccc" with the actual exe path, runs the command,
/// and returns Ok(()) if the process exits successfully or Err on failure.
fn ccc_command_runner(line: &str) -> Result<(), Box<dyn Error>> {
    // 1. Prepare command string
    let trimmed = line.trim();
    let command_line = trimmed.replacen("ccc", PATH_CCC_EXE, 1);
    let mut parts = command_line.split_whitespace();
    let exe = parts.next().unwrap_or(PATH_CCC_EXE);
    let args: Vec<&str> = parts.collect();

    // 2. Check that the executable path exists and is a file
    let exe_path = Path::new(exe);
    if !exe_path.exists() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            format!("executable not found: {}", exe),
        )));
    }

    if VERBOSE.load(std::sync::atomic::Ordering::Relaxed) {
        println!("Running command: {} {:?}", exe, args);
    }

    // 2. Spawn process, convert any I/O error into Box<dyn Error>
    let status = Command::new(exe)
        .args(&args)
        .status()
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

    // 3. Check exit code and wrap failure in Box<dyn Error>
    if status.success() {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("ccc command error: {}", status),
        )))
    }
}

fn diag_command_check_and_run(trimmed_line: &str) -> Result<bool, Box<dyn Error>> {
    if trimmed_line.contains("diag") {
        println!("  - Do you want to run diag:");
        if get_key_entry_y()? == 0 {
            return Ok(true);
        }
        ccc_command_runner(trimmed_line)?;
        return Ok(true);
    }
    // No diag command was detected.
    Ok(false)
}

pub fn ccc_handler(trimmed_line: &str, auto: bool) -> Result<(), Box<dyn Error>> {
    if diag_command_check_and_run(trimmed_line)? {
        return Ok(());
    }
    if !auto {
        println!("  - Press Enter to RUN: {}", trimmed_line);
        press_enter_no_message();
    }
    ccc_command_runner(trimmed_line)?;
    Ok(())
}
