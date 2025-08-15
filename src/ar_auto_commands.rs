use std::error::Error;

use crate::misc::get_key_entry_y;

const COMMAND_KEYWORDS: &[&str] = &[
    "SEMI_AUTO",
    // Add more as needed
];

pub fn check_for_commands(line: &str) -> u32 {
    let trimmed = line.trim();

    if trimmed.starts_with("##") || trimmed.starts_with("#") {
        return 0;
    }

    for &keyword in COMMAND_KEYWORDS {
        if trimmed.contains(keyword) {
            return 1;
        }
    }

    0
}

fn semi_auto_command_handler() -> Result<(), Box<dyn Error>> {
    println!("\nSEMI_AUTO detected.");
    println!("Enter 'y' to proceed with semi-automatic steps, or any other key to skip.");

    let key = get_key_entry_y()?;
    if key == 1 {
        println!("Proceeding with semi-automatic steps...");
        // Add semi-automatic logic here
    } else {
        println!("Skipping semi-automatic steps.");
    }

    Ok(())
}

pub fn command_selector(command: u32) -> Result<(), Box<dyn Error>> {
    if command == 1 {
        if let Err(e) = semi_auto_command_handler() {
            eprintln!("Error in semi-automatic command handler: {}", e);
        }
    } else {
        println!("No auto commands found in instructions.");
    }

    Ok(())
}
