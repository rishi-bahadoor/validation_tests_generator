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
use crate::excel_ops::{convert_csv_to_excel, format_excel_sheet};
use crate::misc::press_enter;
use crate::python_env::sanity_dependencies;
use crate::sanity::{prepend_hash_to_toml, sanity_check_python_scripts, sanity_check_toml};
use crate::test_file_ops::{export_grouped_csv, export_grouped_toml, test_file_filter};
use op_selector::{email_gen, excel_gen, test_run};

const DEFAULT_INSTRUCTION_FILE: &str = "validation_test_instructions.toml";

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

    // Parse groups: Vec<(label, Vec<test_id>)>
    let mut label_groups = Vec::new();
    if !args.groups.is_empty() {
        for raw in &args.groups {
            let mut parts = raw.splitn(2, ':');
            let label = parts.next().unwrap().to_string();
            let ids = parts
                .next()
                .unwrap_or("")
                .split(',')
                .filter(|s| !s.is_empty())
                .map(String::from)
                .collect();
            label_groups.push((label, ids));
        }
    } else if let Some(prio) = &args.priority {
        label_groups.push((prio.clone(), Vec::new()));
    }

    // Apply filter
    let mut grouped_tests = Vec::new();
    for (label, ids) in &label_groups {
        let filtered = test_file_filter(&args.input, ids, &args.priority)?;
        grouped_tests.push((label.clone(), filtered));
    }

    // Export a grouped TOML summary
    export_grouped_toml(&grouped_tests, DEFAULT_INSTRUCTION_FILE)?;
    prepend_hash_to_toml(DEFAULT_INSTRUCTION_FILE)?;
    sanity_check_toml(DEFAULT_INSTRUCTION_FILE)?;

    // CSV → Excel pipeline
    sanity_check_python_scripts()?;
    sanity_dependencies()?;
    let csv_path = export_grouped_csv(DEFAULT_INSTRUCTION_FILE, &args.output)?;
    let xlsx_path = convert_csv_to_excel(&csv_path)?;
    format_excel_sheet(&xlsx_path)?;

    Ok(())
}
