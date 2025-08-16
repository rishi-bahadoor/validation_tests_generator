use std::error::Error;
use toml::Value;

use crate::ar_ccc_commands::ccc_command_runner;
use crate::ar_generic_commands::generic_runner;
use crate::misc::{get_key_entry_y, press_enter_no_message};

const COMMAND_KEYWORDS: &[&str] = &[
    "SEMI_AUTO_CCC",
    "FULL_AUTO_CCC",
    "FULL_AUTO_PANORAMA",
    // Add more as needed
];

fn semi_auto_ccc_handler(instructions: &Vec<Value>) -> Result<(), Box<dyn Error>> {
    println!("\nSEMI_AUTO detected.");
    if get_key_entry_y()? == 0 {
        println!("Skipping automatic steps.");
        return Ok(());
    }
    println!("--------------------------------------------------------------");
    println!("Step by step semi automatic instrucion runner");

    for instr in instructions {
        if let Some(line) = instr.as_str() {
            let trimmed = line.trim();

            if trimmed.starts_with("##") || trimmed.starts_with("#") {
                println!("  - {}", line);
            } else if trimmed.starts_with("ccc") {
                println!("  - Press Enter to RUN: {}", line);
                press_enter_no_message();
                ccc_command_runner(line)?;
            } else {
                generic_runner(line)?;
            }
        }
    }

    Ok(())
}

fn full_auto_ccc_handler(_instructions: &Vec<Value>) -> Result<(), Box<dyn Error>> {
    println!("\nFULL_AUTO detected.");
    if get_key_entry_y()? == 0 {
        println!("Skipping automatic steps.");
        return Ok(());
    }

    // Add full-automatic logic here

    Ok(())
}

pub fn auto_command_selector(
    command: u32,
    instructions: &Vec<Value>,
) -> Result<(), Box<dyn Error>> {
    if command == 1 {
        if let Err(e) = semi_auto_ccc_handler(instructions) {
            eprintln!("Error in semi-automatic command handler: {}", e);
        }
    } else if command == 2 {
        if let Err(e) = full_auto_ccc_handler(instructions) {
            eprintln!("Error in full-automatic command handler: {}", e);
        }
    } else if command == 3 {
        // TODO: add panorama handler.
        println!("In development");
    } else {
        println!("No auto commands found in instructions.");
    }

    Ok(())
}

pub fn check_for_commands(line: &str) -> u32 {
    let trimmed = line.trim();

    for &keyword in COMMAND_KEYWORDS {
        if trimmed.contains(keyword) {
            // TODO: return different values for different commands
            if keyword == "SEMI_AUTO_CCC" {
                return 1;
            } else if keyword == "FULL_AUTO_CCC" {
                return 2;
            } else if keyword == "FULL_AUTO_PANORAMA" {
                return 3;
            }
        }
    }

    0
}
