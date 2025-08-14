## VALIDATION REPORT GENERATION

### Features
- Parse test definitions from a TOML file
- Filter tests by ID or priority (LOW, MEDIUM, HIGH)
- Generate a technician-friendly CSV report
- Includes metadata rows for technician and firmware details

### Using the tool
You can run the tool directly using .\vtg.exe.

### Filter by test ID

example:
- .\vtg.exe --group GROUP_ONE:1.1,1.2 --group GROUP_TWO:1.3,1.4

or

- .\vtg.exe -g GROUP_ONE:1.1,1.2 -g GROUP_TWO:1.3,1.4

### Filter by priority

example:
- .\vtg.exe --priority MEDIUM

### Specify the test list

example:
- .\vtg.exe --input C:\Users\name\Downloads\validation_gen\validation_tests_generator\base_tests_list.toml

### Specify the output file name

example:
- .\vtg.exe --output test.csv


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

### Generating the email template

To generate the email template, ensure that a report named `validation_test_report.xlsx` already exists in the source directory. This file is used as input to create a technician-ready HTML email preview.

The email includes:
- Metadata rows (e.g., technician name, firmware version)
- A formatted test result table with conditional coloring
- Additional sheets like `Technician_Issues` or `Github_Issues` rendered as simple tables

Once ready, run:

- .\vtg.exe --email-gen example.sender@example.com example.recipient@example.com
