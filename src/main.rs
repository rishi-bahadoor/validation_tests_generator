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
use test_file_ops::test_file_filter;

#[derive(Parser, Debug)]
#[command(name = "vtg.exe", version = "1.0")]
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
    if std::env::args().len() == 1 {
        let mut cmd = Args::command();
        let version = cmd.get_version().unwrap_or("unknown");
        println!("\nvtg version: {}", version);
        cmd.print_help()?;
        println!(); // newline
        print!("Press Enter to continue…");
        io::stdout().flush()?; // ensure prompt shows
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?; // wait for Enter
        return Ok(());
    }

    let args = Args::parse();

    if args.email_gen {
        let sender = args.sender_email.as_deref().unwrap();
        let recipient = args.recipient_email.as_deref().unwrap();
        let _ = generate_email_using_python(sender, recipient)?;
        return Ok(());
    }

    // Parse each "label:comma,ids" into (label, Vec<String>)
    let mut label_groups: Vec<(String, Vec<String>)> = Vec::new();

    if !args.groups.is_empty() {
        for raw in &args.groups {
            let mut parts = raw.splitn(2, ':');
            let label = parts
                .next()
                .expect("every group must have a label")
                .to_string();
            let id_list = parts
                .next()
                .unwrap_or("") // in case someone writes "foo:" with no IDs
                .split(',')
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect::<Vec<_>>();
            label_groups.push((label, id_list));
        }
    } else if let Some(ref prio) = args.priority {
        // use the actual priority string as the group label
        label_groups.push((
            prio.clone(),
            Vec::new(), // empty IDs means "all IDs", filtered by that priority
        ));
    }

    let mut grouped_tests: Vec<(String, Vec<_>)> = Vec::new();
    for (label, ids) in &label_groups {
        let filtered = test_file_filter(&args.input, ids, &args.priority)?;
        grouped_tests.push((label.clone(), filtered));
    }

    let csv_path = export_grouped_csv(&grouped_tests, &args.output)?;
    let xlsx_path = convert_csv_to_excel(&csv_path)?;
    format_excel_sheet(&xlsx_path)?;

    let xlsx_path = convert_csv_to_excel(&csv_path)?;
    format_excel_sheet(&xlsx_path)?;
    Ok(())
}
