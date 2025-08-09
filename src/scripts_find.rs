use std::env;
use std::error::Error;
use std::path::PathBuf;

pub fn script_path(script_name: &str) -> Result<PathBuf, Box<dyn Error>> {
    // 1) where is the running exe?
    let exe = env::current_exe()?;
    let exe_dir = exe.parent().ok_or("Failed to get executable directory")?;

    // 2) look under resources/<script_name>
    let path = exe_dir.join("resources").join(script_name);

    // 3) sanity check
    if !path.exists() {
        let curr_dir = env::current_dir()?;
        let path_fallback = curr_dir.join("resources").join(script_name);

        if path_fallback.exists() {
            return Ok(path_fallback);
        }

        return Err(format!("{} not found at {}", script_name, path.display()).into());
    }

    Ok(path)
}
