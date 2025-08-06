use clap::Parser;
use csv::Writer;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
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

    /// Filter by priority: LOW, MEDIUM, or HIGH
    #[arg(short, long)]
    priority: Option<String>,
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
    let file = File::create(path)?;
    let mut buf = BufWriter::new(file);

    // Write metadata rows manually
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

    // Blank line for spacing
    writeln!(buf)?;

    // Wrap BufWriter in csv::Writer for structured test records
    let mut wtr = Writer::from_writer(buf);

    // Write header
    wtr.write_record(&[
        "Test ID",
        "Test Group",
        "Priority",
        "Description",
        "Pass Condition",
        "Status",
        "Notes",
    ])?;

    // Write test records
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
        .filter(|t| {
            let id_match = args.ids.is_empty() || args.ids.contains(&t.test_id);
            let priority_match = match &args.priority {
                Some(p) => t.test_priority.eq_ignore_ascii_case(p),
                None => true,
            };
            id_match && priority_match
        })
        .collect();

    if filtered.is_empty() {
        println!("⚠️ No matching tests found for filters.");
    } else {
        export_to_csv(&filtered, &args.output)?;
        println!("✅ CSV report generated: {}", args.output);
    }

    Ok(())
}
