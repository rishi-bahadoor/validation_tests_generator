use clap::{Parser, Subcommand};

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
        #[arg(short = 's', long = "sender_email")]
        sender_email: String,
        #[arg(short = 'r', long = "recipient_email")]
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
        #[arg(short = 'i', long = "input-instruction-file")]
        input_instruction_file: Option<String>,
    },
    /// Group tests by label and IDs
    IdGroups {
        #[arg(value_name = "LABEL:IDS", num_args = 1..)]
        groups: Vec<String>,
        #[arg(short = 'p', long = "priority")]
        priority: Option<String>,
        #[arg(short = 'i', long = "input-instruction-file")]
        input_instruction_file: Option<String>,
    },
    /// Group tests by priority only
    Priority {
        #[arg(value_name = "PRIORITY")]
        priority: String,
        #[arg(short = 'i', long = "input-instruction-file")]
        input_instruction_file: Option<String>,
    },
}
