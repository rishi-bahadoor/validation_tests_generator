use std::fs::File;
use std::io::{BufRead, BufReader, Read};

/// Struct to hold script metadata
struct ScriptCheck<'a> {
    path: &'a str,
    expected_version: &'a str,
    expected_hash: u32,
}

/// Constants: Add your scripts here
const SCRIPT_CHECKS: &[ScriptCheck] = &[
    ScriptCheck {
        path: "resources/excel_format.py",
        expected_version: "1.1.1",
        expected_hash: 0xB02EE3BA, // Replace with actual hash
    },
    ScriptCheck {
        path: "resources/excel_to_email_template.py",
        expected_version: "1.1.1",
        expected_hash: 0xD327BF4A, // Replace with actual hash
    },
];

/// Helper: Extracts version from first line
fn read_version_from_script(path: &str) -> Result<String, String> {
    let file = File::open(path).map_err(|e| format!("Failed to open '{}': {}", path, e))?;
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

/// Helper: Computes FNV-1a 32-bit hash of file contents
fn compute_fnv1a_32(path: &str) -> Result<u32, String> {
    const FNV_OFFSET_BASIS: u32 = 0x811C9DC5;
    const FNV_PRIME: u32 = 0x01000193;

    let mut file = File::open(path).map_err(|e| format!("Failed to open '{}': {}", path, e))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| format!("Failed to read '{}': {}", path, e))?;

    let mut hash = FNV_OFFSET_BASIS;
    for byte in buffer {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    Ok(hash)
}

/// Operation: Performs sanity check across all scripts
pub fn sanity_check() -> Result<(), String> {
    for check in SCRIPT_CHECKS {
        let actual_version = read_version_from_script(check.path)?;
        if actual_version != check.expected_version {
            return Err(format!(
                "Version mismatch in '{}': expected '{}', found '{}'",
                check.path, check.expected_version, actual_version
            ));
        }

        let actual_hash = compute_fnv1a_32(check.path)?;
        if actual_hash != check.expected_hash {
            println!("!! DEVELOPER WARNING !!");
            println!("Script files have changed.");
            println!(
                "Please update the version number in the script header for {}.",
                check.path
            );
            println!("Then update the new hash value in sanity.rs.");
            return Err(format!(
                "Hash mismatch in '{}': expected 0x{:08X}, found 0x{:08X}",
                check.path, check.expected_hash, actual_hash
            ));
        }
    }
    Ok(())
}
