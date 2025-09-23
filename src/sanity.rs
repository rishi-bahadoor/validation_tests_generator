use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use toml::Value;

const FNV_OFFSET_BASIS: u32 = 0x56544732; // VTG2
const FNV_PRIME: u32 = 0x01565447; // 0x01 VTG

const PREFIX: &str = "# HASH ";

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
        expected_version: "1.1.4",
        expected_hash: 0xB0698043, // Replace with actual hash
    },
    ScriptCheck {
        path: "resources/excel_to_email_template.py",
        expected_version: "1.1.6",
        expected_hash: 0xA06FD7C9, // Replace with actual hash
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
        println!("Incorrect script version: '{}'", path);
        println!("Contact the developer.");
        Err(format!("Incorrect version"))
    }
}

/// Helper: Computes FNV-1a 32-bit hash of file contents
fn compute_fnv1a_32(path: &str) -> Result<u32, String> {
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

fn toml_compute_fnv1a_32(path: &str) -> Result<u32, String> {
    // 1. Read entire file into memory
    let data = fs::read(path).map_err(|e| format!("Failed to open or read '{}': {}", path, e))?;

    // 2. Locate the end of the first line
    let first_nl = match data.iter().position(|&b| b == b'\n') {
        Some(idx) => idx + 1,
        None => {
            // No newline at all: nothing to hash beyond offset basis
            return Ok(FNV_OFFSET_BASIS);
        }
    };

    // 3. From there, locate the end of the second line
    let start = match data[first_nl..].iter().position(|&b| b == b'\n') {
        Some(idx) => first_nl + idx + 1,
        None => {
            // Only one newline in the file: hash nothing past that
            return Ok(FNV_OFFSET_BASIS);
        }
    };

    // 4. Compute FNV-1a hash over data[start..]
    let mut hash = FNV_OFFSET_BASIS;
    for &byte in &data[start..] {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    Ok(hash)
}

fn toml_hash_reader(path: &str) -> Result<u32, String> {
    // 1. Open the file
    let file = File::open(path).map_err(|e| format!("Failed to open '{}': {}", path, e))?;
    let mut reader = BufReader::new(file);

    // 2. Read exactly the first line
    let mut header = String::new();
    reader
        .read_line(&mut header)
        .map_err(|e| format!("Failed to read first line: {}", e))?;
    let header = header.trim();

    // 3. Verify the prefix
    if !header.starts_with(PREFIX) {
        return Err(format!(
            "Unexpected header. Expected prefix `{}`, found `{}`",
            PREFIX, header
        ));
    }

    // 4. Extract and clean the hex portion
    let hex_str = header[PREFIX.len()..].trim_start_matches("0x");

    // 5. Parse as a base-16 u32
    u32::from_str_radix(hex_str, 16)
        .map_err(|e| format!("Failed to parse hex `{}`: {}", hex_str, e))
}

/// Reads a TOML file, computes its FNV-1a-32 checksum,
/// and prepends a `# HASH 0xXXXXXXXX` header before writing it back.
pub fn prepend_hash_to_toml<P: AsRef<Path>>(path: P) -> Result<PathBuf, Box<dyn Error>> {
    // 1. Read the full file as a String
    let content = fs::read_to_string(&path)?;

    // 2. Hash every byte, including any existing header or whitespace
    let mut hash = FNV_OFFSET_BASIS;
    for &b in content.as_bytes() {
        hash ^= b as u32;
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    // 3. Build the new file text with the computed hash at the top
    let new_contents = format!("{}0x{:08X}\n\n{}", PREFIX, hash, content);

    // 4. Overwrite the file and return its path
    fs::write(&path, new_contents)?;
    Ok(path.as_ref().to_path_buf())
}

/// Operation: Performs sanity check across all scripts
pub fn sanity_check_python_scripts() -> Result<(), String> {
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

            // For developer check.
            // println!(
            //     "expected 0x{:08X}, found 0x{:08X}",
            //     check.expected_hash, actual_hash
            // );

            /*
              Update the version number in the script header to indicate new script changes.
              Then update the new VERSION and HASH value in sanity.rs to pass the sanity check.
            */
            return Err(format!(
                "Script files have been tampered with {}.",
                check.path
            ));
        }
    }
    Ok(())
}

pub fn sanity_check_toml(path: &str) -> Result<(), String> {
    // 1) Quick extension check
    let p = Path::new(path);
    match p.extension().and_then(OsStr::to_str) {
        Some(ext) if ext.eq_ignore_ascii_case("toml") => {}
        _ => {
            return Err(format!("Expected a '.toml' file extension, got '{}'", path));
        }
    }

    // 2) Parse‐as‐TOML check
    let contents =
        fs::read_to_string(p).map_err(|e| format!("Failed to read '{}': {}", path, e))?;
    toml::from_str::<Value>(&contents)
        .map_err(|e| format!("Invalid TOML syntax in '{}': {}", path, e))?;

    let actual_hash = toml_compute_fnv1a_32(path)?;
    let expected_hash = toml_hash_reader(path)?;

    // For developer check.
    // println!(
    //     "expected 0x{:08X}, found 0x{:08X}",
    //     expected_hash, actual_hash
    // );

    if actual_hash != expected_hash {
        return Err(format!("Toml file tampered: '{}'.", path));
    }

    Ok(())
}
