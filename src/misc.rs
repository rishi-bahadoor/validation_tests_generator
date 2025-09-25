use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

use crate::commands_ccc::get_config_dump;
use crate::excel_ops::{report_sheet_test_id_fail, report_sheet_test_id_pass};

const EMAIL_ATTACHMENTS_FLDR: &str = "./email_attachments/";

pub fn generate_email_attachments() -> Result<(), Box<dyn Error>> {
    get_config_dump(EMAIL_ATTACHMENTS_FLDR)
}

pub fn press_enter() {
    println!("\nPress Enter to continue...");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}

pub fn press_enter_no_message() {
    println!(">");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}

pub fn get_key_entry_y() -> Result<u32, Box<dyn Error>> {
    println!("Enter 'y' to proceed, or any other key to skip.");
    print!("> ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim().to_string();
    if trimmed == "y" { Ok(1) } else { Ok(0) }
}

pub fn wait_s(seconds: u32) {
    let pb = ProgressBar::new(seconds as u64)
        .with_style(
            ProgressStyle::with_template("    {bar:40.green/yellow} {msg:>12.cyan}").unwrap(),
        )
        .with_message("Waiting...");
    for s in 0..seconds {
        sleep(Duration::from_secs(1));
        pb.inc(1);
        pb.set_message(format!("Timeout {}s\t{seconds}s total", seconds - s));
    }
    pb.finish_with_message("Done");
}

pub fn print_thick_separator() {
    println!("=========================================================================");
}

pub fn print_thin_separator() {
    println!("-------------------------------------------------------------------------");
}

pub fn test_pass_prompt() -> Result<u32, Box<dyn Error>> {
    println!("\nDid the test pass? Press [y] if yes or anything else for no.");
    print!("> ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim().to_string();
    if trimmed == "y" { Ok(1) } else { Ok(0) }
}

pub fn test_pass_fail_prompt<P: AsRef<std::path::Path>>(
    xlsx_path: P,
    test_id: &str,
) -> Result<(), Box<dyn Error>> {
    let ret = test_pass_prompt()?; // returns 1 for pass, 0 for fail

    if ret == 1 {
        let default_note = "The test passed the expected criteria";
        report_sheet_test_id_pass(xlsx_path, test_id, Some(default_note))?;
    } else {
        print!("Enter optional note (or leave blank for default fail note): ");
        io::stdout().flush()?;
        let mut note_input = String::new();
        io::stdin().read_line(&mut note_input)?;
        let note = note_input.trim();

        let final_note = if note.is_empty() {
            "The test did not meet the expected criteria"
        } else {
            note
        };

        report_sheet_test_id_fail(xlsx_path, test_id, Some(final_note))?;
    }

    Ok(())
}

#[macro_export]
macro_rules! print_warn_ln {
    ($($arg:tt)*) => {
        color_print::cprintln!("<yellow>[WARN]</> {}", format!($($arg)*));
    };
}

#[macro_export]
macro_rules! print_help_ln {
    ($($arg:tt)*) => {
        color_print::cprintln!("<blue>[HELPER]</> {}", format!($($arg)*));
    };
}
