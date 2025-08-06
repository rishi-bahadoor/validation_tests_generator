use csv::Writer;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Test {
    test_id: String,
    test_group: String,
    test_priority: String,
    test_description: String,
    pass_condition: String,
    #[allow(dead_code)]
    instructions: Vec<String>, // Parsed but not exported
    #[allow(dead_code)]
    test_authors_and_contact_persons: Vec<String>, // Parsed but not exported
}

#[derive(Debug, Deserialize)]
struct TestList {
    test: Vec<Test>,
}

fn export_to_csv<P: AsRef<Path>>(tests: &[Test], path: P) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(path)?;

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
            "", // Status placeholder
            "", // Notes placeholder
        ])?;
    }

    wtr.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Load TOML file
    let toml_str = fs::read_to_string("tests_list.toml")?;

    // Parse TOML into structs
    let test_list: TestList = toml::from_str(&toml_str)?;

    // Export to CSV
    export_to_csv(&test_list.test, "test_report.csv")?;

    println!("✅ CSV report generated: test_report.csv");
    Ok(())
}
