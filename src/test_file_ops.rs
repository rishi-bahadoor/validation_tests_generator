use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Test {
    pub test_id: String,
    pub test_group: String,
    pub test_priority: String,
    pub test_description: String,
    pub pass_condition: String,
    #[allow(dead_code)]
    pub instructions: Vec<String>,
    #[allow(dead_code)]
    pub test_authors_and_contact_persons: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct TestList {
    test: Vec<Test>,
}

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
            let priority_match = match priority {
                Some(p) => t.test_priority.eq_ignore_ascii_case(p),
                None => true,
            };
            id_match && priority_match
        })
        .collect();

    Ok(filtered)
}
