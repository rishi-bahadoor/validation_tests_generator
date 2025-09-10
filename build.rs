use std::path::Path;
use std::process::{Command, Stdio};

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let dll_path = Path::new(&manifest_dir)
        .join("npcap-sdk")
        .join("Lib")
        .join("x64");

    if !dll_path.exists()
        && let Err(e) = extract_wpcap_to_project()
    {
        println!("cargo:warning=Failed to extract wpcap.dll: {}", e);
    }

    // Tell cargo to look in the project directory for DLLs at runtime
    println!(
        "cargo:rustc-link-search=native={}",
        dll_path.to_str().unwrap()
    );
}

fn extract_wpcap_to_project() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let extracted_dir = Path::new(&manifest_dir).join("npcap-sdk");
    let sdk_zip_path = Path::new(&manifest_dir)
        .join("pc_required_installations")
        .join("npcap-sdk-1.15.zip");

    // Extract directly to project directory
    extract_zip(&sdk_zip_path, &extracted_dir)?;

    println!(
        "cargo:warning=Extracted npcap-sdk to {}",
        extracted_dir.display()
    );
    Ok(())
}

fn extract_zip(zip_path: &Path, extract_to: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure the destination directory exists
    std::fs::create_dir_all(extract_to)?;

    // Validate zip file exists
    if !zip_path.exists() {
        return Err(format!("Zip file not found: {}", zip_path.display()).into());
    }

    println!(
        "cargo:warning=Extracting {} to {}",
        zip_path.display(),
        extract_to.display()
    );

    // Use PowerShell Expand-Archive with LiteralPath
    let powershell_command = format!(
        "try {{ Expand-Archive -LiteralPath '{}' -DestinationPath '{}' -Force; Write-Host 'Extraction completed successfully' }} catch {{ Write-Error $_.Exception.Message; exit 1 }}",
        zip_path.display(),
        extract_to.display()
    );

    let output = Command::new("powershell")
        .arg("-NoProfile") // Don't load PowerShell profile (faster)
        .arg("-ExecutionPolicy")
        .arg("Bypass") // Bypass execution policy restrictions
        .arg("-Command")
        .arg(powershell_command)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);

        return Err(format!(
            "Failed to extract zip file:\nSTDERR: {}\nSTDOUT: {}",
            stderr, stdout
        )
        .into());
    }

    // Verify extraction succeeded by checking if destination has content
    if extract_to.read_dir()?.next().is_none() {
        return Err("Extraction completed but no files found in destination".into());
    }

    println!("cargo:warning=Successfully extracted zip file");
    Ok(())
}
