use std::{error::Error, fs, io, path::Path, process::Command};

use chrono::Local;

use crate::misc::{get_key_entry_y, press_enter_no_message};

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

fn get_ccc_output(args: &str) -> Result<String, Box<dyn Error>> {
    let mut command = Command::new(PATH_CCC_EXE);
    command.arg("get");
    command.args(args.split_whitespace());

    // println!("Executing command: {:?}", command);

    let output = command
        .output()
        .map_err(|e| Box::new(e) as Box<dyn Error>)?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("ccc get command failed: {}", stderr),
        )))
    }
}

fn get_ccc_output_integer(args: &str) -> Result<i32, Box<dyn Error>> {
    let output = get_ccc_output(args)?;
    output
        .trim()
        .split(|c| c == '[' || c == ']')
        .nth(1)
        .ok_or("Failed to extract integer from square brackets")?
        .trim()
        .parse::<i32>()
        .map_err(|e| Box::new(e) as Box<dyn Error>)
}

pub fn factory_init() -> Result<(), Box<dyn Error>> {
    println!("Running factory_init...");

    let serial_number = get_ccc_output_integer("serial_number")?.to_string();

    let line = format!(
        "ccc factory-init --sku 0 --serial-number {} ultra.cepbin",
        serial_number
    );

    println!("Running {}", line);

    ccc_command_runner(&line)
}

pub fn get_config_dump(path: &str) -> Result<(), Box<dyn Error>> {
    if !std::path::Path::new(path).exists() {
        std::fs::create_dir_all(path)?;
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_file()
            && entry
                .file_name()
                .to_string_lossy()
                .starts_with("sensor_config_dump_")
        {
            fs::remove_file(entry.path())?;
        }
    }

    let now = Local::now();
    let timestamp = now.format("[%d_%m_%Y]_[%Hhr_%Mmin]").to_string();

    // Do a get-all to get config at email generation time
    ccc_handler(
        &format!("ccc get-all -o {path}/sensor_config_dump_{timestamp}.txt -d ultra_config.toml"),
        true,
    )
}
