use clap::{CommandFactory, Parser};
use std::io::{self, Write};

mod csv_ops;
mod email_ops;
mod excel_ops;
mod scripts_find;
mod test_file_ops;

use csv_ops::export_grouped_csv;
use email_ops::generate_email_using_python;
use excel_ops::{convert_csv_to_excel, format_excel_sheet};
use test_file_ops::{export_grouped_toml, test_file_filter};

#[derive(Parser, Debug)]
#[command(name = "vtg.exe", version = "1.1")]
#[command(about = "Generates filtered validation test reports")]
pub struct Args {
    /// Input TOML of tests
    #[arg(short, long, default_value = "tests_list.toml")]
    pub input: String,

    /// Output CSV path
    #[arg(short, long, default_value = "test_report.csv")]
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
      help = "Define a group, e.g. --group heat:1.1,1.2",
      num_args = 1..,
    )]
    pub groups: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Show help if no args
    if std::env::args().len() == 1 {
        let mut cmd = Args::command();
        let version = cmd.get_version().unwrap_or("unknown");
        println!("\nvtg version: {}", version);
        cmd.print_help()?;
        println!();
        print!("Press Enter to continue…");
        io::stdout().flush()?;
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        return Ok(());
    }

    let args = Args::parse();

    // Email‐only mode
    if args.email_gen {
        let sender = args.sender_email.as_deref().unwrap();
        let recipient = args.recipient_email.as_deref().unwrap();
        let _ = generate_email_using_python(sender, recipient)?;
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

    // CSV → Excel pipeline
    let csv_path = export_grouped_csv(&grouped_tests, &args.output)?;
    let xlsx_path = convert_csv_to_excel(&csv_path)?;
    format_excel_sheet(&xlsx_path)?;

    // Export a grouped TOML summary
    let toml_out = "test_instructions.toml";
    export_grouped_toml(&grouped_tests, toml_out)?;

    Ok(())
}
