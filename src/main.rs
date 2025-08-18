use clap::{CommandFactory, Parser};

mod ar_auto_commands;
mod ar_ccc_commands;
mod ar_generic_commands;
mod ar_process_vti;
mod email_ops;
mod excel_ops;
mod misc;
mod python_env;
mod sanity;
mod scripts_find;
mod test_file_ops;

use ar_process_vti::ar_process_test_item;
use email_ops::generate_email_using_python;
use excel_ops::{convert_csv_to_excel, format_excel_sheet};
use misc::press_enter;
use python_env::sanity_dependencies;
use sanity::{prepend_hash_to_toml, sanity_check_python_scripts, sanity_check_toml};
use test_file_ops::{export_grouped_csv, export_grouped_toml, test_file_filter};

use crate::ar_ccc_commands::VERBOSE;

const DEFAULT_EXCEL_FILE: &str = "validation_test_report.xlsx";
const DEFAULT_INSTRUCTION_FILE: &str = "validation_test_instructions.toml";
const DEFAULT_CSV_FILE: &str = "validation_test_report.csv";
const DEFAULT_BASE_TOML: &str = "base_tests_list.toml";

#[derive(Parser, Debug)]
#[command(name = "vtg.exe", version = "2.1")]
#[command(about = "Generates filtered validation test reports")]
pub struct Args {
    /// Input TOML of tests
    #[arg(short, long, default_value = DEFAULT_BASE_TOML)]
    pub input: String,

    /// Output CSV path
    #[arg(short, long, default_value = DEFAULT_CSV_FILE)]
    pub output: String,

    /// Optional priority filter
    #[arg(short, long)]
    pub priority: Option<String>,

    /// Generate an email instead of full pipeline
    #[arg(short = 'e', long)]
    pub email_gen: bool,

    /// Sender email address (only with --gen-email)
    #[arg(index = 1, required_if_eq("email_gen", "true"))]
    pub sender_email: Option<String>,

    /// Recipient email address (only with --gen-email)
    #[arg(index = 2, required_if_eq("email_gen", "true"))]
    pub recipient_email: Option<String>,

    /// One or more labeled ID groups like label:1.1,1.2,1.3
    #[arg(
      short = 'g',
      long = "group",
      value_name = "LABEL:IDS",
      help = "Define a group, e.g. --group heat:1.1,1.2 --group volt:2.1,2.2",
      num_args = 1..,
    )]
    pub groups: Vec<String>,

    /// One test at a time
    #[arg(
      short = 't',
      long = "test",
      value_name = "TEST_ID",
      help = "Get the test instructions, e.g. --test 1.1 1.2",
      num_args = 1..,
    )]
    pub test: Vec<String>,

    /// Generate excel from toml
    #[arg(short = 'x', long)]
    pub excel: bool,

    #[arg(long = "verbose", help = "Enable verbose output")]
    pub verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Show help if no args.
    if std::env::args().len() == 1 {
        let mut cmd = Args::command();
        let version = cmd.get_version().unwrap_or("unknown");
        println!("\nvtg version: {}", version);
        cmd.print_help()?;
        println!();
        press_enter();
        return Ok(());
    }

    let args = Args::parse();

    if args.verbose {
        println!("Running in verbose mode");
        VERBOSE.store(true, std::sync::atomic::Ordering::Relaxed);
    }

    // Email‐only mode
    if args.email_gen {
        sanity_check_python_scripts()?;
        sanity_dependencies()?;

        let sender = args.sender_email.as_deref().unwrap();
        let recipient = args.recipient_email.as_deref().unwrap();
        let _ = generate_email_using_python(sender, recipient, DEFAULT_EXCEL_FILE)?;
        return Ok(());
    }

    // Test-only mode
    if !args.test.is_empty() {
        sanity_check_toml(DEFAULT_INSTRUCTION_FILE)?;
        for test_id in &args.test {
            if let Err(e) = ar_process_test_item(DEFAULT_INSTRUCTION_FILE, test_id) {
                eprintln!("Error processing test '{}': {}", test_id, e);
            }
        }
        press_enter();
        return Ok(());
    }

    // Excel generation mode only
    if args.excel {
        sanity_check_python_scripts()?;
        sanity_dependencies()?;
        sanity_check_toml(DEFAULT_INSTRUCTION_FILE)?;
        let csv_path = export_grouped_csv(DEFAULT_INSTRUCTION_FILE, &args.output)?;
        let xlsx_path = convert_csv_to_excel(&csv_path)?;
        format_excel_sheet(&xlsx_path)?;
        return Ok(());
    }

    // Parse groups: Vec<(label, Vec<test_id>)>
    let mut label_groups = Vec::new();
    if !args.groups.is_empty() {
        for raw in &args.groups {
            let mut parts = raw.splitn(2, ':');
            let label = parts.next().unwrap().to_string();
            let ids = parts
                .next()
                .unwrap_or("")
                .split(',')
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect();
            label_groups.push((label, ids));
        }
    } else if let Some(prio) = &args.priority {
        label_groups.push((prio.clone(), Vec::new()));
    }

    // Apply filter
    let mut grouped_tests = Vec::new();
    for (label, ids) in &label_groups {
        let filtered = test_file_filter(&args.input, ids, &args.priority)?;
        grouped_tests.push((label.clone(), filtered));
    }

    // Export a grouped TOML summary
    export_grouped_toml(&grouped_tests, DEFAULT_INSTRUCTION_FILE)?;
    prepend_hash_to_toml(DEFAULT_INSTRUCTION_FILE)?;
    sanity_check_toml(DEFAULT_INSTRUCTION_FILE)?;

    // CSV → Excel pipeline
    sanity_check_python_scripts()?;
    sanity_dependencies()?;
    let csv_path = export_grouped_csv(DEFAULT_INSTRUCTION_FILE, &args.output)?;
    let xlsx_path = convert_csv_to_excel(&csv_path)?;
    format_excel_sheet(&xlsx_path)?;

    Ok(())
}
