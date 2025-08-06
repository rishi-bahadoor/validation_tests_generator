## VALIDATION REPORT GENERATION

### Features
- Parse test definitions from a TOML file
- Filter tests by ID or priority (LOW, MEDIUM, HIGH)
- Generate a technician-friendly CSV report
- Includes metadata rows for technician and firmware details

### Installation
- Ensure you have Rust installed. Then clone and build:
```bash
git clone https://github.com/your-repo/validation_tests_generator.git

cd validation_tests_generator

cargo build --release
```

### Using the tool
You can run the tool directly using cargo run -r -- during development.

### Filter by test ID

example:
- cargo run -r -- --ids 1.1
- cargo run -r -- --ids 1.1,2.1

### Filter by priority

example:
- cargo run -r -- --priority MEDIUM
