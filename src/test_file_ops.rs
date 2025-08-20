use csv::Writer;
use serde::Deserialize;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use toml::Value;

#[derive(Debug, Deserialize)]
pub struct Test {
    pub test_id: String,
    pub test_group: String,
    pub test_priority: String,
    pub test_description: String,
    pub pass_condition: String,
    pub instructions: Vec<String>,
    pub test_authors_and_contact_persons: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct TestList {
    test: Vec<Test>,
}

/// Read the TOML at `input_path`, filter by `ids` and/or `priority`.
pub fn test_file_filter(
    input_path: &str,
    ids: &[String],
    priority: &Option<String>,
) -> Result<Vec<Test>, Box<dyn Error>> {
    // Check if the file exists
    if !std::path::Path::new(input_path).exists() {
        return Err(format!("File does not exist: '{}' ", input_path).into());
    }

    let toml_str = fs::read_to_string(input_path)?;
    let test_list: TestList = toml::from_str(&toml_str)?;

    let filtered = test_list
        .test
        .into_iter()
        .filter(|t| {
            let id_match = ids.is_empty() || ids.contains(&t.test_id);
            let prio_match = match priority {
                Some(p) => t.test_priority.eq_ignore_ascii_case(p),
                None => true,
            };
            id_match && prio_match
        })
        .collect();

    Ok(filtered)
}

/// Write a grouped TOML report, with each instruction and author on its own line.
pub fn export_grouped_toml<P: AsRef<Path>>(
    groups: &[(String, Vec<Test>)],
    out_path: P,
) -> Result<PathBuf, Box<dyn Error>> {
    let mut toml_buf = String::new();

    for (label, tests) in groups {
        // Top-level table for the group
        toml_buf.push_str(&format!("[{}]\n\n", label));

        for test in tests {
            // Array-of-tables header
            toml_buf.push_str(&format!("[[{}.test]]\n", label));

            // Scalar fields
            toml_buf.push_str(&format!("test_id = {:?}\n", test.test_id));
            toml_buf.push_str(&format!("test_group = {:?}\n", test.test_group));
            toml_buf.push_str(&format!("test_priority = {:?}\n", test.test_priority));
            toml_buf.push_str(&format!("test_description = {:?}\n", test.test_description));
            toml_buf.push_str(&format!("pass_condition = {:?}\n", test.pass_condition));

            // Multi-line instructions array
            toml_buf.push_str("instructions = [\n");
            for instr in &test.instructions {
                toml_buf.push_str(&format!("  {:?},\n", instr));
            }
            toml_buf.push_str("]\n");

            // Multi-line authors/contact array
            toml_buf.push_str("test_authors_and_contact_persons = [\n");
            for author in &test.test_authors_and_contact_persons {
                toml_buf.push_str(&format!("  {:?},\n", author));
            }
            toml_buf.push_str("]\n\n");
        }
    }

    fs::write(&out_path, toml_buf)?;
    println!("✅ TOML report generated: {}", out_path.as_ref().display());
    Ok(out_path.as_ref().to_path_buf())
}

pub fn export_grouped_csv<P: AsRef<Path>>(
    toml_path: P,
    output_path: P,
) -> Result<PathBuf, Box<dyn Error>> {
    let toml_str = fs::read_to_string(&toml_path)?;
    let root: Value = toml::from_str(&toml_str)?;

    // 2. Collect groups
    let mut groups: Vec<(String, Vec<Test>)> = Vec::new();
    if let Value::Table(table) = root {
        for (label, section) in table {
            if let Some(Value::Array(arr)) = section.get("test") {
                let tests = arr
                    .clone()
                    .into_iter()
                    .map(|tbl| tbl.try_into())
                    .collect::<Result<_, _>>()?;
                groups.push((label, tests));
            }
        }
    }

    // 3. Open output CSV file
    let file = File::create(&output_path)?;
    let mut buf = BufWriter::new(file);

    // 4. Write fixed metadata header
    for line in &[
        "Technician Name:,",
        "Firmware Type:,",
        "Firmware Version:,",
        "Sensor Serial Number:,",
        "CCC Tool Version:,",
    ] {
        writeln!(buf, "{}", line)?;
    }
    writeln!(buf)?; // blank

    // 5. Hand off to CSV writer
    let mut wtr = Writer::from_writer(buf);

    // 6. Emit each group in TOML order
    for (label, tests) in groups {
        let mut buf = wtr.into_inner()?;
        writeln!(buf)?; // blank
        writeln!(buf, "{}", label)?; // section label
        wtr = Writer::from_writer(buf);

        // table header
        wtr.write_record(&[
            "Test ID",
            "Test Group",
            "Priority",
            "Description",
            "Status",
            "Notes",
            "Frequency",
        ])?;

        // rows for this group
        for t in tests {
            wtr.write_record(&[
                &t.test_id,
                &t.test_group,
                &t.test_priority,
                &t.test_description,
                "",
                "",
                "",
            ])?;
        }
    }

    // 7. Finalize
    let mut buf = wtr.into_inner()?;
    buf.flush()?;
    println!(
        "✅ CSV report generated: {}",
        output_path.as_ref().display()
    );
    Ok(output_path.as_ref().to_path_buf())
}

pub fn extract_test_ids<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Box<dyn Error>> {
    // Read the TOML file into a string
    let toml_str = fs::read_to_string(path)?;

    // Parse the string into a toml::Value
    let root = toml_str.parse::<Value>()?;

    let mut ids = Vec::new();

    // Ensure the top-level is a table
    if let Value::Table(tables) = root {
        // Iterate over each top-level table (e.g. "1_POINT_CLOUD")
        for (_, group_val) in tables {
            if let Value::Table(group) = group_val {
                // Look for an array-of-tables named "test"
                if let Some(Value::Array(tests)) = group.get("test") {
                    for test_val in tests {
                        if let Value::Table(test_tbl) = test_val {
                            // Extract the "test_id" field if it exists
                            if let Some(Value::String(id)) = test_tbl.get("test_id") {
                                ids.push(id.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(ids)
}
