use clap::Parser;

mod ar_auto_commands;
mod ar_ccc_commands;
mod ar_generic_commands;
mod ar_process_vti;
mod args;
mod email_ops;
mod excel_ops;
mod misc;
mod op_selector;
mod python_env;
mod sanity;
mod scripts_find;
mod test_file_ops;

use crate::args::Args;

use crate::args::Command;
use crate::op_selector::{email_gen, excel_gen, group_tests_id, group_tests_priority, test_run};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.command {
        Command::Email {
            sender_email,
            recipient_email,
        } => {
            email_gen(&sender_email, &recipient_email)?;
        }
        Command::Test { test_ids } => {
            test_run(&test_ids)?;
        }
        Command::Excel { output } => {
            excel_gen(&output)?;
        }
        Command::Group {
            groups,
            priority,
            input,
            output,
        } => {
            group_tests_id(&groups, &priority, &input, &output)?;
        }
        Command::Priority {
            priority,
            input,
            output,
        } => {
            group_tests_priority(&priority, &input, &output)?;
        }
    }

    Ok(())
}
