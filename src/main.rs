use clap::Parser;
use csv::Writer;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;

/// CLI arguments
#[derive(Parser, Debug)]
#[command(name = "Validation Test Generator")]
#[command(about = "Generates filtered CSV reports from TOML test definitions")]
struct Args {
    /// Path to the TOML file
    #[arg(short, long, default_value = "tests_list.toml")]
    input: String,

    /// Path to the output CSV file
    #[arg(short, long, default_value = "test_report.csv")]
    output: String,

    /// List of test IDs to include in the report
    #[arg(short, long, value_delimiter = ',')]
    ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Test {
    test_id: String,
    test_group: String,
    test_priority: String,
    test_description: String,
    pass_condition: String,
    #[allow(dead_code)]
    instructions: Vec<String>,
    #[allow(dead_code)]
    test_authors_and_contact_persons: Vec<String>,
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
    let args = Args::parse();

    let toml_str = fs::read_to_string(&args.input)?;
    let test_list: TestList = toml::from_str(&toml_str)?;

    let filtered: Vec<Test> = test_list
        .test
        .into_iter()
        .filter(|t| args.ids.contains(&t.test_id))
        .collect();

    if filtered.is_empty() {
        println!("⚠️ No matching tests found for IDs: {:?}", args.ids);
    } else {
        export_to_csv(&filtered, &args.output)?;
        println!("✅ CSV report generated: {}", args.output);
    }

    Ok(())
}
