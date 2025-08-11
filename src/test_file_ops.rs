use serde::Deserialize;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize)]
pub struct Test {
    pub test_id: String,
    pub test_group: String,
    pub test_priority: String,
    pub test_description: String,
    pub pass_condition: String,
    pub instructions: Vec<String>,
    pub test_authors_and_contact_persons: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct TestList {
    test: Vec<Test>,
}

/// Read the TOML at `input_path`, filter by `ids` and/or `priority`.
pub fn test_file_filter(
    input_path: &str,
    ids: &[String],
    priority: &Option<String>,
) -> Result<Vec<Test>, Box<dyn Error>> {
    let toml_str = fs::read_to_string(input_path)?;
    let test_list: TestList = toml::from_str(&toml_str)?;

    let filtered = test_list
        .test
        .into_iter()
        .filter(|t| {
            let id_match = ids.is_empty() || ids.contains(&t.test_id);
            let prio_match = match priority {
                Some(p) => t.test_priority.eq_ignore_ascii_case(p),
                None => true,
            };
            id_match && prio_match
        })
        .collect();

    Ok(filtered)
}

/// Write a grouped TOML report, with each instruction and author on its own line.
pub fn export_grouped_toml<P: AsRef<Path>>(
    groups: &[(String, Vec<Test>)],
    out_path: P,
) -> Result<PathBuf, Box<dyn Error>> {
    let mut toml_buf = String::new();

    for (label, tests) in groups {
        // Top-level table for the group
        toml_buf.push_str(&format!("[{}]\n\n", label));

        for test in tests {
            // Array-of-tables header
            toml_buf.push_str(&format!("[[{}.test]]\n", label));

            // Scalar fields
            toml_buf.push_str(&format!("test_id = {:?}\n", test.test_id));
            toml_buf.push_str(&format!("test_description = {:?}\n", test.test_description));

            // Multi-line instructions array
            toml_buf.push_str("instructions = [\n");
            for instr in &test.instructions {
                toml_buf.push_str(&format!("  {:?},\n", instr));
            }
            toml_buf.push_str("]\n");

            // Multi-line authors/contact array
            toml_buf.push_str("test_authors_and_contact_persons = [\n");
            for author in &test.test_authors_and_contact_persons {
                toml_buf.push_str(&format!("  {:?},\n", author));
            }
            toml_buf.push_str("]\n\n");
        }
    }

    fs::write(&out_path, toml_buf)?;
    println!("✅ TOML report generated: {}", out_path.as_ref().display());
    Ok(out_path.as_ref().to_path_buf())
}
