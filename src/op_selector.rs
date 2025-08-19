use std::error::Error;

use crate::ar_process_vti::ar_process_test_item;
use crate::email_ops::generate_email_using_python;
use crate::excel_ops::{convert_csv_to_excel, format_excel_sheet};
use crate::misc::press_enter;
use crate::python_env::sanity_dependencies;
use crate::sanity::prepend_hash_to_toml;
use crate::sanity::{sanity_check_python_scripts, sanity_check_toml};
use crate::test_file_ops::export_grouped_csv;
use crate::test_file_ops::{export_grouped_toml, test_file_filter};

const DEFAULT_EXCEL_FILE: &str = "validation_test_report.xlsx";
const DEFAULT_INSTRUCTION_FILE: &str = "validation_test_instructions.toml";

pub fn email_gen(sender_email: &String, recipient_email: &String) -> Result<(), Box<dyn Error>> {
    sanity_check_python_scripts()?;
    sanity_dependencies()?;

    let sender = sender_email.as_str();
    let recipient = recipient_email.as_str();
    let _ = generate_email_using_python(sender, recipient, DEFAULT_EXCEL_FILE)?;
    Ok(())
}

pub fn test_run(test_ids: &Vec<String>) -> Result<(), Box<dyn Error>> {
    sanity_check_toml(DEFAULT_INSTRUCTION_FILE)?;
    for test_id in test_ids {
        println!("======================================================================");
        if let Err(e) = ar_process_test_item(DEFAULT_INSTRUCTION_FILE, test_id) {
            eprintln!("Error processing test '{}': {}", test_id, e);
        }
    }
    press_enter();
    Ok(())
}

pub fn excel_gen(output: &String) -> Result<(), Box<dyn Error>> {
    sanity_check_python_scripts()?;
    sanity_dependencies()?;
    sanity_check_toml(DEFAULT_INSTRUCTION_FILE)?;
    let csv_path = export_grouped_csv(DEFAULT_INSTRUCTION_FILE, output)?;
    let xlsx_path = convert_csv_to_excel(&csv_path)?;
    format_excel_sheet(&xlsx_path)?;
    Ok(())
}

pub fn group_tests_id(
    groups: &Vec<String>,
    priority: &Option<String>,
    input: &String,
    output: &String,
) -> Result<(), Box<dyn Error>> {
    // Parse groups: Vec<(label, Vec<test_id>)>
    let mut label_groups = Vec::new();
    let mut label: String;
    let mut ids: Vec<String>;

    for raw in groups {
        let mut parts = raw.splitn(2, ':');
        label = parts.next().unwrap().to_string();
        ids = parts
            .next()
            .unwrap_or("")
            .split(',')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();
        label_groups.push((label, ids));
    }

    // Apply filter
    let mut grouped_tests = Vec::new();
    for (label, ids) in &label_groups {
        let filtered = test_file_filter(input.as_str(), ids, priority)?;
        grouped_tests.push((label.clone(), filtered));
    }

    // Export a grouped TOML summary
    export_grouped_toml(&grouped_tests, DEFAULT_INSTRUCTION_FILE)?;
    prepend_hash_to_toml(DEFAULT_INSTRUCTION_FILE)?;
    sanity_check_toml(DEFAULT_INSTRUCTION_FILE)?;

    // CSV → Excel pipeline
    sanity_check_python_scripts()?;
    sanity_dependencies()?;
    let csv_path = export_grouped_csv(DEFAULT_INSTRUCTION_FILE, output.as_str())?;
    let xlsx_path = convert_csv_to_excel(&csv_path)?;
    format_excel_sheet(&xlsx_path)?;

    Ok(())
}

pub fn group_tests_priority(
    priority: &String,
    input: &String,
    output: &String,
) -> Result<(), Box<dyn Error>> {
    let mut label_groups = Vec::new();

    // Create Option<String> from &String
    let priority_opt = if priority.trim().is_empty() {
        None
    } else {
        Some(priority.clone())
    };

    if let Some(ref prio) = priority_opt {
        label_groups.push((prio.clone(), Vec::new()));
    }

    // Apply filter
    let mut grouped_tests = Vec::new();
    for (label, ids) in &label_groups {
        let filtered = test_file_filter(input.as_str(), ids, &priority_opt)?;
        grouped_tests.push((label.clone(), filtered));
    }

    // Export a grouped TOML summary
    export_grouped_toml(&grouped_tests, DEFAULT_INSTRUCTION_FILE)?;
    prepend_hash_to_toml(DEFAULT_INSTRUCTION_FILE)?;
    sanity_check_toml(DEFAULT_INSTRUCTION_FILE)?;

    // CSV → Excel pipeline
    sanity_check_python_scripts()?;
    sanity_dependencies()?;
    let csv_path = export_grouped_csv(DEFAULT_INSTRUCTION_FILE, output.as_str())?;
    let xlsx_path = convert_csv_to_excel(&csv_path)?;
    format_excel_sheet(&xlsx_path)?;

    Ok(())
}
