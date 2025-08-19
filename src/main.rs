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
        Command::Test {
            test_ids,
            input_instruction_file,
        } => {
            let is_file_custom = match input_instruction_file {
                Some(_) => true,
                None => false,
            };

            test_run(&test_ids, &input_instruction_file, is_file_custom)?;
        }
        Command::Excel {} => {
            excel_gen()?;
        }
        Command::IdGroups {
            groups,
            priority,
            input,
        } => {
            group_tests_id(&groups, &priority, &input)?;
        }
        Command::Priority { priority, input } => {
            group_tests_priority(&priority, &input)?;
        }
    }

    Ok(())
}
