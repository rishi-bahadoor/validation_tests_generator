use clap::Parser;

mod csv_ops;
mod email_ops;
mod excel_ops;
mod test_file_ops;

use csv_ops::export_to_csv;
use email_ops::generate_email_using_python;
use excel_ops::{convert_csv_to_excel, format_excel_sheet};
use test_file_ops::test_file_filter;

#[derive(Parser, Debug)]
#[command(name = "Validation Test Generator")]
#[command(about = "Generates filtered CSV reports from TOML test definitions")]
pub struct Args {
    #[arg(short = 'i', long = "input", default_value = "tests_list.toml")]
    pub input: String,

    #[arg(short = 'o', long = "output", default_value = "test_report.csv")]
    pub output: String,

    #[arg(short = 'd', long = "ids")]
    pub ids: Vec<String>,

    #[arg(short = 'p', long = "priority")]
    pub priority: Option<String>,

    /// Generate an email instead of running the full report pipeline
    #[arg(long)]
    pub gen_email: bool,

    /// Sender email address (only with --gen-email)
    #[arg(index = 1, required_if_eq("gen_email", "true"))]
    pub sender_email: Option<String>,

    /// Recipient email address (only with --gen-email)
    #[arg(index = 2, required_if_eq("gen_email", "true"))]
    pub recipient_email: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.gen_email {
        let sender = args.sender_email.as_deref().unwrap();
        let recipient = args.recipient_email.as_deref().unwrap();
        let _ = generate_email_using_python(sender, recipient)?;
        return Ok(());
    }

    let filtered_list = test_file_filter(&args.input, &args.ids, &args.priority)?;
    let csv_path = export_to_csv(&filtered_list, &args.output)?;
    let xlsx_path = convert_csv_to_excel(&csv_path)?;
    format_excel_sheet(&xlsx_path)?;
    Ok(())
}
