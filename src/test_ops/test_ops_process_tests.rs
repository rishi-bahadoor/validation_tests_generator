use std::error::Error;
use std::fs;
use toml::Value;

use crate::test_ops::test_ops_process_instructions::{get_instruction_type, instructions_runner};

pub fn process_fetched_instructions(
    test_id: &str,
    instructions: &Vec<Value>,
) -> Result<(), Box<dyn Error>> {
    let mut auto_command: Option<&'static str> = None;

    for instr in instructions {
        if let Some(line) = instr.as_str() {
            println!("  - {}", line);

            if auto_command.is_none() {
                match get_instruction_type(line)? {
                    Some(cmd) => auto_command = Some(cmd),
                    None => {
                        // No auto command found in this line; continue
                    }
                }
            }
        }
    }

    if let Some(cmd) = auto_command {
        instructions_runner(test_id, cmd, instructions)?;
    }

    Ok(())
}

pub fn ar_process_test_item(file: &str, user_input_test_id: &str) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(file)?;
    let parsed: Value = content.parse::<Value>()?;

    for (group_name, group_value) in parsed.as_table().ok_or("Invalid TOML structure")? {
        if let Some(tests) = group_value.get("test").and_then(|v| v.as_array()) {
            for test in tests {
                let test_id = test.get("test_id").and_then(|v| v.as_str()).unwrap_or("");
                if test_id == user_input_test_id {
                    println!("Test Group: {}", group_name);
                    println!("Test ID: {}", test_id);
                    let pass_condition = test
                        .get("pass_condition")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    println!("Pass Condition: {}", pass_condition);
                    println!("Instructions:");
                    if let Some(instructions) = test.get("instructions").and_then(|v| v.as_array())
                    {
                        process_fetched_instructions(test_id, instructions)?;
                    }
                    return Ok(());
                }
            }
        }
    }

    println!(
        "Test ID '{}' not found in file '{}'",
        user_input_test_id, file
    );

    Ok(())
}

pub fn ar_print_test_item(file: &str, user_input_test_id: &str) -> Result<(), Box<dyn Error>> {
    // Read and parse
    let content = fs::read_to_string(file)?;
    let parsed: Value = content.parse::<Value>()?;
    let table = parsed
        .as_table()
        .ok_or("Invalid TOML structure: expected a table at top level")?;

    // First pass: compute max widths
    let mut max_group_len = 0;
    let mut max_test_id_len = 0;
    for (group_name, group_value) in table {
        if let Some(tests) = group_value.get("test").and_then(|v| v.as_array()) {
            for test in tests {
                if let Some(id) = test.get("test_id").and_then(|v| v.as_str()) {
                    max_group_len = max_group_len.max(group_name.len());
                    max_test_id_len = max_test_id_len.max(id.len());
                }
            }
        }
    }
    // Add padding
    let col1_width = max_group_len + 2;
    let col2_width = max_test_id_len + 2;

    // Second pass: find and print the matching test
    for (group_name, group_value) in table {
        if let Some(tests) = group_value.get("test").and_then(|v| v.as_array()) {
            for test in tests {
                let test_id = test.get("test_id").and_then(|v| v.as_str()).unwrap_or("");
                if test_id == user_input_test_id {
                    let test_description = test
                        .get("test_description")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    println!(
                        "{:<col1$}{:<col2$}{}",
                        group_name,
                        test_id,
                        test_description,
                        col1 = col1_width,
                        col2 = col2_width
                    );
                    return Ok(());
                }
            }
        }
    }

    println!(
        "Test ID '{}' not found in file '{}'",
        user_input_test_id, file
    );
    Ok(())
}
