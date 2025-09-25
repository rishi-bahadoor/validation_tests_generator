use clap::{CommandFactory, Parser};

// macro_use modules must be declared before modules that use it.
#[macro_use]
mod misc;
#[macro_use(bytes_u32)]
extern crate dhcp4r;

mod ar_auto_commands;
mod ar_ccc_commands;
mod ar_generic_commands;
mod ar_panorama_commands;
mod ar_process_vti;
mod dhcp_server;
mod email_ops;
mod excel_ops;
mod interface;
mod op_selector;
mod pcap_ops;
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
            email_name,
            input_excel_file,
            sender_email,
            recipient_email,
            bypass_generation,
        } => {
            email_gen(
                &email_name,
                &input_excel_file,
                &sender_email,
                &recipient_email,
                bypass_generation,
            )?;
        }
        Command::Test {
            test_ids,
            input_instruction_file,
            input_excel_file,
        } => {
            test_run(test_ids, &input_instruction_file, &input_excel_file)?;
        }
        Command::Excel {
            input_instruction_file,
        } => {
            excel_gen(&input_instruction_file)?;
        }
        Command::IdGroups {
            groups,
            priority,
            input_instruction_file,
            output_name,
        } => {
            group_tests_id(&groups, &priority, &input_instruction_file, &output_name)?;
        }
        Command::Priority {
            priority,
            input_instruction_file,
            output_name,
        } => {
            group_tests_priority(&priority, &input_instruction_file, &output_name)?;
        }
    }

    Ok(())
}
