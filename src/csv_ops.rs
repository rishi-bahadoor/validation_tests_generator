use csv::Writer;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::test_file_ops::Test;

pub fn export_to_csv<P: AsRef<Path>>(tests: &[Test], path: P) -> Result<(), Box<dyn Error>> {
    if tests.is_empty() {
        println!("⚠️ No matching tests found for filters.");
        return Ok(());
    }

    let file = File::create(&path)?; // Borrow to avoid move
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
    Ok(())
}
