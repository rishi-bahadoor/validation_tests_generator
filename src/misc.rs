pub fn press_enter() {
    println!("Press Enter to continue...");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}
