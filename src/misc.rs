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
    println!("Enter 'y' to proceed with semi-automatic steps, or any other key to skip.");
    print!("> ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim().to_string();
    if trimmed == "y" { Ok(1) } else { Ok(0) }
}

pub fn wait_s(seconds: u32) {
    let timer_1 = seconds;
    println!("Timer: {} seconds", timer_1);
    for n in 0..(timer_1 + 1) {
        sleep(Duration::from_millis(1000));
        print!("\r{:3} ", timer_1 - n); // Use padding for consistent width
        io::stdout().flush().unwrap(); // Ensure output is shown immediately
    }
    println!(); // Move to a new line after the loop finishes
}
