use std::{error::Error, io, path::Path};

const PATH_PANORAMA_CLI_EXE: &str = "./panorama_cli.exe";

fn panorama_cli_runner(line: &str) -> Result<(), Box<dyn Error>> {
    let trimmed = line.trim();
    let command_line = trimmed.replacen("panorama", &format!("{PATH_PANORAMA_CLI_EXE} run -t"), 1);
    let mut parts = command_line.split_whitespace();
    let exe = parts.next().unwrap_or(PATH_PANORAMA_CLI_EXE);
    let args: Vec<&str> = parts.collect();

    // 2. Check that the executable path exists and is a file
    let exe_path = Path::new(exe);
    if !exe_path.exists() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            format!("executable not found: {}", exe),
        )));
    }

    let status = std::process::Command::new(exe)
        .args(&args)
        .status()
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

    if status.success() {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("panorama_cli command error: {}", status),
        )))
    }
}

pub fn panorama_cli_handler(trimmed_line: &str) -> Result<(), Box<dyn Error>> {
    // Implementation for handling panorama_cli commands
    panorama_cli_runner(trimmed_line)?;
    Ok(())
}
