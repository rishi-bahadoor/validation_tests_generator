use indicatif::{ProgressBar, ProgressStyle};
use std::error::Error;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

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
        .with_style(ProgressStyle::with_template("{bar:40.green/yellow} {msg:>12.blue}").unwrap())
        .with_message("Waiting...");
    for s in 0..seconds {
        sleep(Duration::from_secs(1));
        pb.inc(1);
        pb.set_message(format!("Timeout {}s\t{seconds}s total", seconds - s));
    }
    pb.finish_with_message("Done");
}
