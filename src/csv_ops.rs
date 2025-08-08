use csv::Writer;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use crate::test_file_ops::Test;

/*
Depricated: Use `export_grouped_csv` instead.

pub fn export_to_csv<P: AsRef<Path>>(tests: &[Test], path: P) -> Result<PathBuf, Box<dyn Error>> {
    if tests.is_empty() {
        println!("⚠️ No matching tests found for filters.");
        return Ok(path.as_ref().to_path_buf());
    }

    let file = File::create(&path)?;
    let mut buf = BufWriter::new(file);

    let metadata = vec![
        "Technician Name:,",
        "Firmware Type:,",
        "Firmware Version:,",
        "Sensor Serial Number:,",
        "CCC Tool Version:,",
    ];

    for line in metadata {
        writeln!(buf, "{}", line)?;
    }

    writeln!(buf)?; // Blank line after metadata

    let mut wtr = Writer::from_writer(buf);

    wtr.write_record(&[
        "Test ID",
        "Test Group",
        "Priority",
        "Description",
        "Pass Condition",
        "Status",
        "Notes",
    ])?;

    for test in tests {
        wtr.write_record(&[
            &test.test_id,
            &test.test_group,
            &test.test_priority,
            &test.test_description,
            &test.pass_condition,
            "",
            "",
        ])?;
    }

    wtr.flush()?;
    println!("✅ CSV report generated: {}", path.as_ref().display());
    Ok(path.as_ref().to_path_buf())
}

*/

pub fn export_grouped_csv<P: AsRef<Path>>(
    groups: &[(String, Vec<Test>)],
    path: P,
) -> Result<PathBuf, Box<dyn Error>> {
    let file = File::create(&path)?;
    let mut buf = BufWriter::new(file);

    // Write your common metadata
    for line in &[
        "Technician Name:,",
        "Firmware Type:,",
        "Firmware Version:,",
        "Sensor Serial Number:,",
        "CCC Tool Version:,",
    ] {
        writeln!(buf, "{}", line)?;
    }
    writeln!(buf)?;

    let mut wtr = Writer::from_writer(buf);

    // For each group: write a blank line, the label as a comment, then the test table
    for (label, tests) in groups {
        // Section header
        // Blank line + label row
        let mut buf = wtr.into_inner()?;
        writeln!(buf)?; // single blank line
        writeln!(buf, "{}", label)?; // just “ONE” or “TWO”
        wtr = Writer::from_writer(buf);

        // Table header
        wtr.write_record(&[
            "Test ID",
            "Test Group",
            "Priority",
            "Description",
            "Pass Condition",
            "Status",
            "Notes",
        ])?;

        // Table rows
        for t in tests {
            wtr.write_record(&[
                &t.test_id,
                &t.test_group,
                &t.test_priority,
                &t.test_description,
                &t.pass_condition,
                "",
                "",
            ])?;
        }
    }

    let mut buf = wtr.into_inner()?;
    buf.flush()?;
    println!("✅ CSV report generated: {}", path.as_ref().display());
    Ok(path.as_ref().to_path_buf())
}
