use std::error::Error;
use toml::Value;

use crate::ar_ccc_commands::{ccc_handler, factory_init};
use crate::ar_generic_commands::generic_runner;
use crate::ar_panorama_commands::panorama_cli_handler;
use crate::misc::{get_key_entry_y, print_thin_separator, wait_s};
use crate::pcap_ops::PcapInstance;

const COMMAND_KEYWORDS: &[&str] = &[
    "SEMI_AUTO",
    "FULL_AUTO",
    // Add more as needed
];

fn event_timed(trimmed_line: &str) -> Result<(), Box<dyn Error>> {
    // Split and collect all whitespace-separated tokens
    let args: Vec<&str> = trimmed_line.split_whitespace().collect();

    // 1. Validate we have at least 4 tokens: [program, timeout, do_period, ...command]
    if args.len() < 4 {
        return Err("Usage: event_timed <timeout_secs> <do_period> <command...>".into());
    }

    // 2. Safely fetch and parse `timeout_secs`
    let timeout: u32 = args
        .get(1)
        .ok_or("Missing <timeout_secs>")?
        .parse()
        .map_err(|e| format!("Invalid timeout '{}': {}", args[1], e))?;

    // 3. Safely fetch and parse `do_period`
    let do_period: u32 = args
        .get(2)
        .ok_or("Missing <do_period>")?
        .parse()
        .map_err(|e| format!("Invalid do_period '{}': {}", args[2], e))?;

    if timeout < do_period {
        return Err("Timeout cannot be less than the period".into());
    }

    let mut cycle_cntr = timeout / do_period;
    let time_modulus = timeout % do_period;
    if time_modulus > 0 {
        cycle_cntr += 1;
    }

    // 4. Everything from index 3 onward is the actual command
    let command_line = args[3..].join(" ");

    // 5. Core timed loop
    println!(
        "Timed event loop for: {} seconds, at {} second intervals. Event: {}",
        timeout, do_period, command_line
    );
    while cycle_cntr > 0 {
        wait_s(do_period);
        cycle_cntr -= 1;
        ccc_handler(&command_line, true)?;
    }

    Ok(())
}

fn instruction_handler(
    test_id: &str,
    instructions: &Vec<Value>,
    auto: bool,
) -> Result<(), Box<dyn Error>> {
    let mut pcap_instance = PcapInstance::new(test_id);
    pcap_instance.start();
    for instr in instructions {
        if let Some(line) = instr.as_str() {
            let trimmed = line.trim();

            if trimmed.starts_with("##") || trimmed.starts_with("#") {
                println!("  - {}", trimmed);
            } else if trimmed.starts_with("ccc") {
                ccc_handler(trimmed, auto)?;
            } else if trimmed.starts_with("event_timed") {
                event_timed(trimmed)?;
            } else if trimmed.starts_with("factory_init") {
                factory_init()?;
            } else if trimmed.starts_with("panorama") {
                panorama_cli_handler(trimmed)?;
            } else {
                generic_runner(trimmed)?;
            }
        }
    }
    pcap_instance.stop();
    Ok(())
}

pub fn auto_command_selector(
    test_id: &str,
    command: &str,
    instructions: &Vec<Value>,
) -> Result<(), Box<dyn Error>> {
    match command {
        "SEMI_AUTO" => {
            println!("\nSEMI_AUTO detected.");
            if get_key_entry_y()? == 0 {
                println!("Skipping automatic steps.");
                return Ok(());
            }
            print_thin_separator();
            println!("Step by step semi automatic instruction runner");
            if let Err(e) = instruction_handler(test_id, instructions, false) {
                eprintln!("Error in semi-automatic command handler: {}", e);
            }
        }
        "FULL_AUTO" => {
            println!("\nFULL_AUTO detected.");
            if get_key_entry_y()? == 0 {
                println!("Skipping automatic steps.");
                return Ok(());
            }
            print_thin_separator();
            println!("Automatic instruction runner");
            if let Err(e) = instruction_handler(test_id, instructions, true) {
                eprintln!("Error in full-automatic command handler: {}", e);
            }
        }
        _ => {
            println!("No auto commands found in instructions.");
        }
    }

    Ok(())
}

pub fn check_for_auto_commands(line: &str) -> Result<Option<&'static str>, Box<dyn Error>> {
    let trimmed = line.trim();

    let fields = trimmed
        .split_whitespace()
        .filter(|&field| !field.is_empty() && field != ("##"))
        .collect::<Vec<&str>>();

    if fields.is_empty() || fields.len() != 1 {
        // line doesn't have format `## <command> ##`
        return Ok(None);
    }

    for &keyword in COMMAND_KEYWORDS {
        if fields[0] == keyword {
            return Ok(Some(keyword));
        }
    }

    Ok(None)
}
