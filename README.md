## VALIDATION REPORT GENERATION

### Features
- Parse test definitions from a TOML file
- Filter tests by ID or priority (LOW, MEDIUM, HIGH)
- Generate a technician-friendly CSV report
- Includes metadata rows for technician and firmware details

### Using the tool
You can run the tool directly using .\vtg.exe.

### Generate report template by test ID groups

example:
- .\vtg.exe --group GROUP_ONE:1.1,1.2 --group GROUP_TWO:1.3,1.4

or

- .\vtg.exe -g GROUP_ONE:1.1,1.2 -g GROUP_TWO:1.3,1.4

###  Generate report template by priority

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

### Instruction commands for test writing
There are some key-words that can be used in test instructions.

- wait_s [UINT_32_VALUE] : waits for UINT_32_VALUE seconds before moving on to
the next instruction.
- wait_e : waits until the user presses the 'Enter Key' to move on to the next
instruction.
- event_timed [UINT_32_TIMEOUT] [UINT_32_PERIOD] [COMMAND_STRING_SPLITS] :
starts an event loop for a total of UINT_32_TIMEOUT seconds to run
COMMAND_STRING_SPLITS every UINT_32_PERIOD seconds.
- ccc [ARGS...] : runs a ccc command.

examples:

``` toml
instructions = [
  "## Comment of instruction 1 ##",
  "wait_s 60",
  "## Do something ##",
  "wait_e",
  "## Do something ##",
  "ccc get-all",
  "event_timed 60 10 ccc list-sensors",
]
```

### Excel Report generation

This functionality generates an excel report template using the filtered
validation_test_instructions.toml.

To generate the sheet, use the command: --excel or -x

example:

- .\vtg.exe --excel
- .\vtg.exe -x

### Instruction types and using the validation test instructions toml

This functionality reads the generated filtered toml list for a specific test 
and prints the instructions on the terminal for that test. If the test has some
form of automation, the user will be prompted to use it. There are various
types of automation levels:

- SEMI_AUTO : allows for step by step execution of the instruction steps
with ccc tool as the main sensor communication medium.
- FULL_AUTO : allows automated execution of the instruction steps with
ccc tool as the main sensor communication medium. (In development)
- FULL_AUTO_PANORAMA : allows for automatically running a panorama test.
 (In development)

example:

``` toml
instructions = [
  "## SEMI_AUTO ##",
  "## Comment of instruction 1 ##",
  "wait_s 60",
  "## Do something ##",
  "wait_e",
  "## Do something ##",
  "ccc get-all",
  "event_timed 60 10 ccc list-sensors",
]
```

If a test as no level of automation, the instruction will simply just be printed
on the terminal.

To run a test from the list of tests generated, use the command: --test or -t

example:

- .\vtg.exe --test 1.1

### Generating the email template

To generate the email template, ensure that a report named `validation_test_report.xlsx` already exists in the source directory. This file is used as input to create a technician-ready HTML email preview.

The email includes:
- Metadata rows (e.g., technician name, firmware version)
- A formatted test result table with conditional coloring
- Additional sheets like `Technician_Issues` or `Github_Issues` rendered as simple tables

Once ready, run:

- .\vtg.exe --email-gen example.sender@example.com example.recipient@example.com
