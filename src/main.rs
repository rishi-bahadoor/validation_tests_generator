use clap::{CommandFactory, Parser};

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
use crate::misc::press_enter;
use op_selector::{email_gen, excel_gen, group_tests_id, group_tests_priority, test_run};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Show help if no args.
    if std::env::args().len() == 1 {
        let mut cmd = Args::command();
        let version = cmd.get_version().unwrap_or("unknown");
        println!("\nvtg version: {}", version);
        cmd.print_help()?;
        println!();
        press_enter();
        return Ok(());
    }

    let args = Args::parse();

    // Email‐only mode
    if args.email_gen {
        email_gen(args)?;
        return Ok(());
    }

    // Test-only mode
    if !args.test.is_empty() {
        test_run(args)?;
        return Ok(());
    }

    // Excel generation mode only
    if args.excel {
        excel_gen(args)?;
        return Ok(());
    }

    if !args.groups.is_empty() {
        group_tests_id(args)?;
        return Ok(());
    }

    if args.priority.as_ref().map_or(false, |p| !p.is_empty()) {
        group_tests_priority(args)?;
        return Ok(());
    }
    Ok(())
}
