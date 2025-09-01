use std::error::Error;

use crate::ar_process_vti::{ar_print_test_item, ar_process_test_item};
use crate::email_ops::generate_email_using_python;
use crate::excel_ops::{convert_csv_to_excel, format_excel_sheet};
use crate::misc::{
    generate_email_attachments, get_key_entry_y, press_enter, print_thick_separator,
};
use crate::python_env::sanity_dependencies;
use crate::sanity::prepend_hash_to_toml;
use crate::sanity::{sanity_check_python_scripts, sanity_check_toml};
use crate::test_file_ops::{
    export_grouped_csv, export_grouped_toml, extract_test_ids, test_file_filter,
};

const DEFAULT_EXCEL_FILE: &str = "validation_test_report.xlsx";
const DEFAULT_INSTRUCTION_FILE: &str = "validation_test_instructions.toml";
const DEFAULT_CSV_FILE: &str = "validation_test_report.csv";
const DEFAULT_BASE_TOML: &str = "base_tests_list.toml";

pub fn email_gen(
    sender_email: &String,
    recipient_email: &String,
    bypass_generation: bool,
) -> Result<(), Box<dyn Error>> {
    // Sanity check the python scripts used for excel sheet operations.
    sanity_check_python_scripts()?;
    sanity_dependencies()?;

    if !bypass_generation {
        println!("Generating email attachments...");
        generate_email_attachments()?;
    }

    // Generate the email template.
    let sender = sender_email.as_str();
    let recipient = recipient_email.as_str();
    let _ = generate_email_using_python(sender, recipient, DEFAULT_EXCEL_FILE)?;
    Ok(())
}

pub fn test_run(
    test_ids: Option<Vec<String>>,
    input_instruction_file: &Option<String>,
) -> Result<(), Box<dyn Error>> {
    // Determine if the file is custom
    let is_file_custom = input_instruction_file.is_some();
    let is_ids_provided = test_ids.is_some();

    // Resolve file path
    let file_path: &str = input_instruction_file
        .as_deref()
        .unwrap_or(DEFAULT_INSTRUCTION_FILE);

    // Skip this sanity check if the input is a custom file.
    // We can remove the skip and check all if all scripts are intended to be
    // non tampering.
    if !is_file_custom {
        sanity_check_toml(file_path)?;
    }

    // Extract test IDs if none were provided
    let ids_to_run = match test_ids {
        Some(ids) => ids,
        None => extract_test_ids(file_path)?,
    };

    if !is_ids_provided {
        for test_id in ids_to_run.clone() {
            if let Err(e) = ar_print_test_item(file_path, &test_id) {
                eprintln!("Error processing test '{}': {}", test_id, e);
            }
        }
        print_thick_separator();
        println!("Do you want to run all tests?");
        if get_key_entry_y()? == 0 {
            return Ok(());
        }
    }

    // Process each test ID
    for test_id in ids_to_run {
        print_thick_separator();
        if let Err(e) = ar_process_test_item(file_path, &test_id) {
            eprintln!("Error processing test '{}': {}", test_id, e);
        }
    }

    press_enter();
    Ok(())
}

pub fn excel_gen(input_instruction_file: &Option<String>) -> Result<(), Box<dyn Error>> {
    // Set flag to skip sanity check if input is a custom file.
    let is_file_custom = input_instruction_file.is_some();

    // Extract &str from Option<String>
    let file_path: &str = input_instruction_file
        .as_deref()
        .unwrap_or(DEFAULT_INSTRUCTION_FILE);

    // Skip this sanity check if the input is a custom file.
    // We can remove the skip and check all if all scripts are intended to be
    // non tampering.
    if !is_file_custom {
        sanity_check_toml(file_path)?; // Now passes &str
    }

    // Sanity check the python scripts used for excel sheet operations.
    sanity_check_python_scripts()?;
    sanity_dependencies()?;

    // Perform the excel generation.
    let csv_path = export_grouped_csv(file_path, DEFAULT_CSV_FILE)?;
    let xlsx_path = convert_csv_to_excel(&csv_path)?;
    format_excel_sheet(&xlsx_path)?;
    Ok(())
}

pub fn group_tests_id(
    groups: &Vec<String>,
    priority: &Option<String>,
    input_base: &Option<String>,
    output_name: &Option<String>,
) -> Result<(), Box<dyn Error>> {
    // Extract &str from Option<String>
    let file_path: &str = input_base.as_deref().unwrap_or(DEFAULT_BASE_TOML);

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
        let filtered = test_file_filter(file_path, ids, priority)?;
        grouped_tests.push((label.clone(), filtered));
    }

    // Generate output file names from Option<String>
    let output_toml_owned: String = match output_name {
        Some(name) => format!("{}.toml", name),
        None => DEFAULT_INSTRUCTION_FILE.to_string(),
    };
    let output_csv_owned: String = match output_name {
        Some(name) => format!("{}.csv", name),
        None => DEFAULT_CSV_FILE.to_string(),
    };

    let output_toml_file: &String = &output_toml_owned;
    let output_csv_file: &String = &output_csv_owned;

    // Export a grouped TOML summary
    export_grouped_toml(&grouped_tests, output_toml_file)?;
    prepend_hash_to_toml(output_toml_file)?;
    sanity_check_toml(output_toml_file)?;

    // CSV → Excel pipeline
    sanity_check_python_scripts()?;
    sanity_dependencies()?;
    let csv_path = export_grouped_csv(output_toml_file, output_csv_file)?;
    let xlsx_path = convert_csv_to_excel(&csv_path)?;
    format_excel_sheet(&xlsx_path)?;

    Ok(())
}

pub fn group_tests_priority(
    priority: &String,
    input_base: &Option<String>,
    output_name: &Option<String>,
) -> Result<(), Box<dyn Error>> {
    // Extract &str from Option<String>
    let file_path: &str = input_base.as_deref().unwrap_or(DEFAULT_BASE_TOML);

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
        let filtered = test_file_filter(file_path, ids, &priority_opt)?;
        grouped_tests.push((label.clone(), filtered));
    }

    // Generate output file names from Option<String>
    let output_toml_owned: String = match output_name {
        Some(name) => format!("{}.toml", name),
        None => DEFAULT_INSTRUCTION_FILE.to_string(),
    };
    let output_csv_owned: String = match output_name {
        Some(name) => format!("{}.csv", name),
        None => DEFAULT_CSV_FILE.to_string(),
    };

    let output_toml_file: &String = &output_toml_owned;
    let output_csv_file: &String = &output_csv_owned;

    // Export a grouped TOML summary
    export_grouped_toml(&grouped_tests, output_toml_file)?;
    prepend_hash_to_toml(output_toml_file)?;
    sanity_check_toml(output_toml_file)?;

    // CSV → Excel pipeline
    sanity_check_python_scripts()?;
    sanity_dependencies()?;
    let csv_path = export_grouped_csv(output_toml_file, output_csv_file)?;
    let xlsx_path = convert_csv_to_excel(&csv_path)?;
    format_excel_sheet(&xlsx_path)?;

    Ok(())
}
