use clap::Parser;
mod csv_ops;
mod test_file_ops;

use csv_ops::export_to_csv;
use test_file_ops::test_file_filter;

/// CLI arguments
#[derive(Parser, Debug)]
#[command(name = "Validation Test Generator")]
#[command(about = "Generates filtered CSV reports from TOML test definitions")]
struct Args {
    #[arg(short, long, default_value = "tests_list.toml")]
    input: String,

    #[arg(short, long, default_value = "test_report.csv")]
    output: String,

    #[arg(short, long, value_delimiter = ',')]
    ids: Vec<String>,

    #[arg(short, long)]
    priority: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let filtered_list = test_file_filter(&args.input, &args.ids, &args.priority)?;
    export_to_csv(&filtered_list, &args.output)?;
    Ok(())
}
