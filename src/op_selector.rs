use std::error::Error;

use crate::ar_process_vti::ar_process_test_item;
use crate::args::Args;
use crate::email_ops::generate_email_using_python;
use crate::excel_ops::{convert_csv_to_excel, format_excel_sheet};
use crate::misc::press_enter;
use crate::python_env::sanity_dependencies;
use crate::sanity::{sanity_check_python_scripts, sanity_check_toml};
use crate::test_file_ops::export_grouped_csv;

const DEFAULT_EXCEL_FILE: &str = "validation_test_report.xlsx";
const DEFAULT_INSTRUCTION_FILE: &str = "validation_test_instructions.toml";

pub fn email_gen(args: Args) -> Result<(), Box<dyn Error>> {
    sanity_check_python_scripts()?;
    sanity_dependencies()?;

    let sender = args.sender_email.as_deref().unwrap();
    let recipient = args.recipient_email.as_deref().unwrap();
    let _ = generate_email_using_python(sender, recipient, DEFAULT_EXCEL_FILE)?;
    Ok(())
}

pub fn test_run(args: Args) -> Result<(), Box<dyn Error>> {
    sanity_check_toml(DEFAULT_INSTRUCTION_FILE)?;
    for test_id in &args.test {
        if let Err(e) = ar_process_test_item(DEFAULT_INSTRUCTION_FILE, test_id) {
            eprintln!("Error processing test '{}': {}", test_id, e);
        }
    }
    press_enter();
    Ok(())
}

pub fn excel_gen(args: Args) -> Result<(), Box<dyn Error>> {
    sanity_check_python_scripts()?;
    sanity_dependencies()?;
    sanity_check_toml(DEFAULT_INSTRUCTION_FILE)?;
    let csv_path = export_grouped_csv(DEFAULT_INSTRUCTION_FILE, &args.output)?;
    let xlsx_path = convert_csv_to_excel(&csv_path)?;
    format_excel_sheet(&xlsx_path)?;
    Ok(())
}
