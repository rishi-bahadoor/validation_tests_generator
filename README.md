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


### Updating the test_list.toml
The test list is done using a toml file type.

Ensure that additional test items are added with the correct format:

The current format is shown below:

``` toml
[[test]]
test_id = "Appropriate test ID, ex: 1.1"
test_group = "Appropriate test group, ex: 1:GROUP_ONE"
test_priority = "Appropriate priority, ex: LOW or MEDIUM or HIGH"
test_description = "Appropriate description of test, ex: Check that something specific happens when something is done"
pass_condition = "Appropriate pass condition, ex: Something specific happens or does not happen"
instructions = [
  "## Comment of instruction 1 ##",
  "## Comment of instruction 2 ##",
  "specific commands for instruction 2, ex: ccc set x=y,a=b",
  "## Comment of instruction 3 ##"",
  "some peripheral task, ex: turn relay off and then on again",
  "## Some reporting task, ex: Observe a specific parameter for x minutes ##"
]
test_authors_and_contact_persons = [
  "The writer(s) of a specific test item, ex: John Doe, Mary Sue",
]
```