use clap::Parser;
mod csv_ops;
mod excel_ops;
mod test_file_ops;

use csv_ops::export_to_csv;
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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let filtered_list = test_file_filter(&args.input, &args.ids, &args.priority)?;
    let csv_path = export_to_csv(&filtered_list, &args.output)?;
    let xlsx_path = convert_csv_to_excel(&csv_path)?;
    format_excel_sheet(&xlsx_path)?;
    Ok(())
}
