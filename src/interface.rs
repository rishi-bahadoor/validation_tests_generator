use clap::{Parser, Subcommand};

const DEFAULT_CSV_FILE: &str = "validation_test_report.csv";
const DEFAULT_BASE_TOML: &str = "base_tests_list.toml";

#[derive(Parser, Debug)]
#[command(
    name = "vtg",
    version = "2.1",
    about = "Generates filtered validation test reports"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Generate email report
    EmailGen {
        sender_email: String,
        recipient_email: String,
    },
    /// Run specific test instructions
    Test {
        #[arg(value_name = "TEST_ID")]
        test_ids: Vec<String>,
        #[arg(short = 'i', long = "input-instruction-file")]
        input_instruction_file: Option<String>,
    },
    /// Generate Excel report from grouped CSV
    Excel {
        #[arg(short = 'o', long = "output-excel-file", default_value = DEFAULT_CSV_FILE)]
        output: String,
    },
    /// Group tests by label and IDs
    IdGroups {
        #[arg(short = 'g', long = "group", value_name = "LABEL:IDS", num_args = 1..)]
        groups: Vec<String>,
        #[arg(short = 'p', long = "priority")]
        priority: Option<String>,
        #[arg(short = 'i', long = "input-instruction-file", default_value = DEFAULT_BASE_TOML)]
        input: String,
    },
    /// Group tests by priority only
    Priority {
        #[arg(short = 'p', long = "priority")]
        priority: String,
        #[arg(short = 'i', long = "input-instruction-file", default_value = DEFAULT_BASE_TOML)]
        input: String,
    },
}
