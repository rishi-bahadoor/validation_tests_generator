use rust_xlsxwriter::{Format, Workbook};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::supporters::scripts_find::script_path;

pub fn convert_csv_to_excel<P: AsRef<Path>>(csv_path: P) -> Result<PathBuf, Box<dyn Error>> {
    // Prepare output path
    let xlsx_path = csv_path.as_ref().with_extension("xlsx");
    let xlsx_filename = xlsx_path.to_str().ok_or("Invalid output path")?;

    // Create workbook with filename
    let mut workbook = Workbook::new(xlsx_filename);
    let worksheet = workbook.add_worksheet();

    // Optional: basic format (can be customized later)
    let format = Format::new();

    // Read CSV line by line
    let file = File::open(&csv_path)?;
    let reader = BufReader::new(file);

    for (row_idx, line) in reader.lines().enumerate() {
        let line = line?;
        let cells: Vec<&str> = line.split(',').collect();

        for (col_idx, cell) in cells.iter().enumerate() {
            worksheet.write_string(row_idx as u32, col_idx as u16, cell, &format)?;
        }
    }

    workbook.close()?;
    fs::remove_file(&csv_path)?;
    println!("Excel file created: {}", xlsx_path.display());

    Ok(xlsx_path)
}

pub fn format_excel_sheet<P: AsRef<Path>>(xlsx_path: P) -> Result<(), Box<dyn Error>> {
    let script = script_path("excel_format.py")?;

    let status = Command::new("python")
        .arg(script)
        .arg("format") // <-- mode argument
        .arg(xlsx_path.as_ref())
        .status()?;

    if !status.success() {
        return Err("Python script failed to format Excel sheet".into());
    }

    println!("Excel formatting complete.");
    Ok(())
}

pub fn update_test_status<P: AsRef<Path>>(
    xlsx_path: P,
    test_id: &str,
    new_status: &str,
    notes: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let script = script_path("excel_format.py")?;

    let mut cmd = Command::new("python");
    cmd.arg(script)
        .arg("update")
        .arg(xlsx_path.as_ref())
        .arg(test_id)
        .arg(new_status);

    if let Some(n) = notes {
        cmd.arg(n);
    }

    let status = cmd.status()?;

    if !status.success() {
        return Err(format!("Failed to update status for Test ID '{}'", test_id).into());
    }

    println!("Status updated for Test ID '{}'", test_id);
    Ok(())
}

pub fn report_sheet_test_id_pass<P: AsRef<Path>>(
    xlsx_path: P,
    test_id: &str,
    notes: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    update_test_status(xlsx_path, test_id, "Pass", notes)
}

pub fn report_sheet_test_id_fail<P: AsRef<Path>>(
    xlsx_path: P,
    test_id: &str,
    notes: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    update_test_status(xlsx_path, test_id, "Fail", notes)
}
