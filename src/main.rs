use clap::{CommandFactory, Parser};

mod ar_auto_commands;
mod ar_ccc_commands;
mod ar_generic_commands;
mod ar_process_vti;
mod email_ops;
mod excel_ops;
mod interface;
mod misc;
mod op_selector;
mod python_env;
mod sanity;
mod scripts_find;
mod test_file_ops;

use crate::interface::Cli;

use crate::interface::Command;
use crate::misc::press_enter;
use crate::op_selector::{email_gen, excel_gen, group_tests_id, group_tests_priority, test_run};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Show help if no arguments are passed
    if std::env::args().len() == 1 {
        let mut cmd = Cli::command();
        println!("\nvtg version: {}", cmd.get_version().unwrap_or("unknown"));
        cmd.print_help()?;
        println!();
        press_enter();
        return Ok(());
    }

    let args = Cli::parse();

    match args.command {
        Command::EmailGen {
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
        Command::IdGroups {
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
