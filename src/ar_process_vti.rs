use std::error::Error;
use std::fs;
use toml::Value;

use crate::ar_auto_commands::{auto_command_selector, check_for_auto_commands};

pub fn process_fetched_instructions(instructions: &Vec<Value>) -> Result<(), Box<dyn Error>> {
    let mut auto_command: Option<&'static str> = None;

    for instr in instructions {
        if let Some(line) = instr.as_str() {
            println!("  - {}", line);

            if auto_command.is_none() {
                match check_for_auto_commands(line)? {
                    Some(cmd) => auto_command = Some(cmd),
                    None => {
                        // No auto command found in this line; continue
                    }
                }
            }
        }
    }

    if let Some(cmd) = auto_command {
        auto_command_selector(cmd, instructions)?;
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
                        process_fetched_instructions(instructions)?;
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
