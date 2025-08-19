use clap::Parser;

const DEFAULT_CSV_FILE: &str = "validation_test_report.csv";
const DEFAULT_BASE_TOML: &str = "base_tests_list.toml";

#[derive(Parser, Debug)]
#[command(name = "vtg.exe", version = "2.1")]
#[command(about = "Generates filtered validation test reports")]
pub struct Args {
    #[arg(short, long, default_value = DEFAULT_BASE_TOML)]
    pub input: String,

    #[arg(short, long, default_value = DEFAULT_CSV_FILE)]
    pub output: String,

    #[arg(short, long)]
    pub priority: Option<String>,

    #[arg(short = 'e', long)]
    pub email_gen: bool,

    #[arg(index = 1, required_if_eq("email_gen", "true"))]
    pub sender_email: Option<String>,

    #[arg(index = 2, required_if_eq("email_gen", "true"))]
    pub recipient_email: Option<String>,

    #[arg(
        short = 'g',
        long = "group",
        value_name = "LABEL:IDS",
        help = "Define a group, e.g. --group heat:1.1,1.2 --group volt:2.1,2.2",
        num_args = 1..,
    )]
    pub groups: Vec<String>,

    #[arg(
        short = 't',
        long = "test",
        value_name = "TEST_ID",
        help = "Get the test instructions, e.g. --test 1.1 1.2",
        num_args = 1..,
    )]
    pub test: Vec<String>,

    #[arg(short = 'x', long)]
    pub excel: bool,
}
