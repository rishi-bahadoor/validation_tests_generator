use std::fs;
use toml::Value;

pub fn check_for_ccc(line: &str) {
    if line.contains("ccc ") {
        println!("    → ccc command detected");
    }
}

pub fn ar_process_test_item(file: &str, user_input_test_id: &str) {
    let content = match fs::read_to_string(file) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("File not found: {}", file);
            return;
        }
    };

    let parsed: Value = match content.parse::<Value>() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to parse TOML: {}", e);
            return;
        }
    };

    let mut found = false;

    for (group_name, group_value) in parsed.as_table().unwrap() {
        if let Some(tests) = group_value.get("test").and_then(|v| v.as_array()) {
            for test in tests {
                let test_id = test.get("test_id").and_then(|v| v.as_str()).unwrap_or("");
                if test_id == user_input_test_id {
                    println!("\nTest Group: {}", group_name);
                    println!("Test ID: {}", test_id);
                    println!("Instructions:");
                    if let Some(instructions) = test.get("instructions").and_then(|v| v.as_array())
                    {
                        for instr in instructions {
                            if let Some(line) = instr.as_str() {
                                println!("  - {}", line);
                                check_for_ccc(line);
                            }
                        }
                    }
                    found = true;
                }
            }
        }
    }

    if !found {
        println!(
            "Test ID '{}' not found in file '{}'",
            user_input_test_id, file
        );
    }
}
