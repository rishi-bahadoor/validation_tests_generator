use std::error::Error;
use toml::Value;

use crate::misc::get_key_entry_y;

const COMMAND_KEYWORDS: &[&str] = &[
    "SEMI_AUTO",
    "FULL_AUTO",
    // Add more as needed
];

fn semi_auto_command_handler(_instructions: &Vec<Value>) -> Result<(), Box<dyn Error>> {
    println!("\nSEMI_AUTO detected.");
    if get_key_entry_y()? == 0 {
        println!("Skipping automatic steps.");
        return Ok(());
    }

    // Add semi-automatic logic here

    Ok(())
}

fn full_auto_command_handler(_instructions: &Vec<Value>) -> Result<(), Box<dyn Error>> {
    println!("\nFULL_AUTO detected.");
    if get_key_entry_y()? == 0 {
        println!("Skipping automatic steps.");
        return Ok(());
    }

    // Add semi-automatic logic here

    Ok(())
}

pub fn auto_command_selector(
    command: u32,
    instructions: &Vec<Value>,
) -> Result<(), Box<dyn Error>> {
    if command == 1 {
        if let Err(e) = semi_auto_command_handler(instructions) {
            eprintln!("Error in semi-automatic command handler: {}", e);
        }
    } else if command == 2 {
        if let Err(e) = full_auto_command_handler(instructions) {
            eprintln!("Error in full-automatic command handler: {}", e);
        }
    } else {
        println!("No auto commands found in instructions.");
    }

    Ok(())
}

pub fn check_for_commands(line: &str) -> u32 {
    let trimmed = line.trim();

    if trimmed.starts_with("##") || trimmed.starts_with("#") {
        return 0;
    }

    for &keyword in COMMAND_KEYWORDS {
        if trimmed.contains(keyword) {
            // TODO: return different values for different commands
            if keyword == "SEMI_AUTO" {
                return 1;
            } else if keyword == "FULL_AUTO" {
                return 2;
            }
        }
    }

    0
}
