use std::error::Error;
use std::io::{self, Write};

pub fn press_enter() {
    println!("\nPress Enter to continue...");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}

pub fn get_key_entry_y() -> Result<u32, Box<dyn Error>> {
    print!("> ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim().to_string();
    if trimmed == "y" { Ok(1) } else { Ok(0) }
}
