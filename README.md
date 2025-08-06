## VALIDATION REPORT GENERATION

### Features
- Parse test definitions from a TOML file
- Filter tests by ID or priority (LOW, MEDIUM, HIGH)
- Generate a technician-friendly CSV report
- Includes metadata rows for technician and firmware details

### Using the tool
You can run the tool directly using cargo run -r -- during development.

### Filter by test ID

example:
- cargo run -r -- --ids 1.1
- cargo run -r -- --ids 1.1,2.1

### Filter by priority

example:
- cargo run -r -- --priority MEDIUM

### Specify the test list

example:
- cargo run -r -- --input C:\Users\rishi\Downloads\validation_gen\validation_tests_generator\tests_list.toml

### Specify the output file name

example:
- cargo run -r -- --output test.csv
