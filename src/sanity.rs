use std::fs::File;
use std::io::{BufRead, BufReader};

/// Struct to hold script path and expected version
struct ScriptCheck<'a> {
    path: &'a str,
    expected_version: &'a str,
}

/// Constant list of scripts to check
const SCRIPT_CHECKS: &[ScriptCheck] = &[
    ScriptCheck {
        path: "resources/excel_format.py",
        expected_version: "1.1.1",
    },
    ScriptCheck {
        path: "resources/excel_email.py",
        expected_version: "1.1.1",
    },
    // Add more scripts here as needed
];

/// Helper: Extracts version from first line of script
fn read_version_from_script(path: &str) -> Result<String, String> {
    let file = File::open(path).map_err(|e| format!("Failed to open script '{}': {}", path, e))?;
    let mut reader = BufReader::new(file);

    let mut first_line = String::new();
    reader
        .read_line(&mut first_line)
        .map_err(|e| format!("Failed to read first line of '{}': {}", path, e))?;

    let version_prefix = "# VERSION ";
    if first_line.starts_with(version_prefix) {
        Ok(first_line.trim()[version_prefix.len()..].to_string())
    } else {
        Err(format!("Malformed or missing version line in '{}'", path))
    }
}

/// Operation: Performs sanity check across all scripts
pub fn sanity_check() -> Result<(), String> {
    for check in SCRIPT_CHECKS {
        let actual = read_version_from_script(check.path)?;
        if actual != check.expected_version {
            return Err(format!(
                "Version mismatch in '{}': expected '{}', found '{}'",
                check.path, check.expected_version, actual
            ));
        }
    }
    Ok(())
}
