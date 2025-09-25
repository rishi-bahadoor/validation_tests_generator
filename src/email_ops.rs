// src/email_ops.rs

use std::error::Error;
use std::process::Command;

use crate::scripts_find::script_path;

pub fn generate_email_using_python(
    email_name: &str,
    sender: &str,
    recipient: &str,
    excel_file: &str,
) -> Result<(), Box<dyn Error>> {
    // Check if the excel file exists
    if !std::path::Path::new(excel_file).exists() {
        return Err(format!("Excel file does not exist: '{}' ", excel_file).into());
    }

    // resolve the .py location
    let script = script_path("excel_to_email_template.py")?;

    let status = Command::new("python")
        .arg(script)
        .arg(email_name)
        .arg(sender)
        .arg(recipient)
        .arg(excel_file)
        .status()?;

    if !status.success() {
        return Err("Python script failed to generate email".into());
    }

    println!("Email `.eml` generated.");
    Ok(())
}
