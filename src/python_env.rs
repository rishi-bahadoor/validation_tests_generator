use std::env::consts::OS;
use std::io::{self, Write};
use std::process::{Command, Stdio};

/// Try to find an available Python executable on PATH.
fn find_python_executable() -> Option<&'static str> {
    let candidates = ["python3", "python", "py"];
    for &cmd in &candidates {
        if Command::new(cmd)
            .arg("--version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
        {
            return Some(cmd);
        }
    }
    None
}

/// Prompt the user with a yes/no question. Default is yes.
fn prompt_yes_no(question: &str) -> io::Result<bool> {
    print!("{} [Y/n]: ", question);
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().is_empty() || buf.to_lowercase().starts_with('y'))
}

/// Attempt to install Python using the system package manager.
fn install_python() -> io::Result<()> {
    println!("\nNo Python interpreter found. Attempting to install…");

    match OS {
        "linux" => {
            // Debian/Ubuntu
            let status = Command::new("sudo").args(&["apt-get", "update"]).status()?;
            if !status.success() {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to run apt-get update",
                ));
            }
            let status = Command::new("sudo")
                .args(&["apt-get", "install", "-y", "python3", "python3-pip"])
                .status()?;
            if status.success() {
                println!("Python installed via apt-get.");
                Ok(())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to install Python via apt-get",
                ))
            }
        }
        "macos" => {
            // Homebrew
            let status = Command::new("brew").args(&["install", "python"]).status()?;
            if status.success() {
                println!("Python installed via Homebrew.");
                Ok(())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to install Python via brew",
                ))
            }
        }
        "windows" => {
            // Use winget or guide to python.org
            if Command::new("winget")
                .args(&["install", "--id", "Python.Python.3"])
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
            {
                println!("Python installed via winget.");
                Ok(())
            } else {
                println!(
                    "Please download and install Python from https://www.python.org/downloads/"
                );
                Err(io::Error::new(io::ErrorKind::Other, "Python not installed"))
            }
        }
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            "Unsupported OS for automatic Python install",
        )),
    }
}

/// Returns true if `python -c "import module"` succeeds.
fn is_module_installed(py: &str, module: &str) -> bool {
    Command::new(py)
        .arg("-c")
        .arg(format!("import {}", module))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Installs a Python module using `python -m pip install <module>`.
fn install_module(py: &str, module: &str) -> io::Result<()> {
    let status = Command::new(py)
        .args(&["-m", "pip", "install", module])
        .status()?;
    if status.success() {
        println!("`{}` installed successfully.", module);
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to install `{}`", module),
        ))
    }
}

/// Ensure Python is present, or install it.
fn ensure_python() -> io::Result<&'static str> {
    if let Some(py) = find_python_executable() {
        return Ok(py);
    }

    if prompt_yes_no("Python not found. Install now?")? {
        install_python()?;
        // Re-check for python after install
        find_python_executable()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Python still not found"))
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Python is required but not installed",
        ))
    }
}

/// Top-level check: Python → openpyxl + pywin32
pub fn sanity_dependencies() -> io::Result<()> {
    let python = ensure_python()?;

    // Check openpyxl
    if is_module_installed(python, "openpyxl") {
        println!("✅ `openpyxl` is installed.");
    } else if prompt_yes_no("`openpyxl` not found. Install now?")? {
        install_module(python, "openpyxl")?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "`openpyxl` is required but not installed",
        ));
    }

    // Check pywin32
    if is_module_installed(python, "win32com.client") {
        println!("✅ `pywin32` is installed.");
    } else if prompt_yes_no("`pywin32` not found. Install now?")? {
        install_module(python, "pywin32")?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "`pywin32` is required but not installed",
        ));
    }

    Ok(())
}
