## VALIDATION REPORT GENERATION

### Features
- Generate grouped sets of tests in TOML format, filter tests by ID or priority (LOW, MEDIUM, HIGH)
- Protection against script files tampering
- protection against grouped test toml tampering
- Generate a technician-friendly Excel report, Includes metadata rows for technician and firmware details
- Validation test runner
- Validation email template generator
- Capture pcap files from test runs
- Automated sensor system configuration dump

### Using the tool
From vtg version 2.2 onward, the user machine will be required to have npcap installed.

A default npcap installation executable for windows can be found at /pc_required_installations/npcap-1.83.exe

Or you can use the following internet link to download the appropriate file: https://npcap.com/

If this is not installed on the user machine, an error message for missing wpcap.dll may be displayed when running the executable
and running the app from a command line will have no results or prints.

You can run the tool directly using .\vtg.exe.

### Generate report template by test ID groups

example:
- .\vtg.exe id-groups 1_POINT_CLOUD:1.1,1.2 GROUP_TWO:1.3,1.4

Filter a certain [PRIORITY] from the entered IDs.
- .\vtg.exe id-groups 1_POINT_CLOUD:1.1,1.2 GROUP_TWO:1.3,1.4 -p HIGH

Specify the output files name
- .\vtg.exe id-groups 1_POINT_CLOUD:1.1,1.2 GROUP_TWO:1.3,1.4 -o my_report_example

###  Generate report template by priority

example:
- .\vtg.exe --priority MEDIUM

Specify the output files name
- .\vtg.exe --priority MEDIUM -o my_report_example

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
- ccc [ARGS...] : runs a ccc command.
- event_timed [UINT_32_TIMEOUT] [UINT_32_PERIOD] [COMMAND_STRING_SPLITS] :
starts an event loop for a total of UINT_32_TIMEOUT seconds to run
COMMAND_STRING_SPLITS every UINT_32_PERIOD seconds.

examples:

``` toml
instructions = [
  "## Comment of instruction 1 ##",
  "wait_s 60",
  "## Do something ##",
  "wait_e",
  "## Do something ##",
  "ccc get-all",
  "## Do an event every 10s for 60s total ##",
  "event_timed 60 10 ccc list-sensors",
]
```

### Excel Report generation

This functionality generates an excel report template using the filtered
validation_test_instructions.toml.

To generate the sheet, use the command: --excel

example:

- .\vtg.exe --excel

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

### Running tests

To run a test from the list of tests generated, use the command: --test

example:

Single run-
- .\vtg.exe --test 1.1

Multi run-
- .\vtg.exe --test 1.1 1.2 1.3

Run all in instruction file
- .\vtg.exe --test

Specify an instruction toml file
- .\vtg.exe --test -i Path/To/Instruction.toml

### Pcap capturing

This tool can automatically capture pcap files during the test run.

The pcap files are generated into the directory /pcaps.

Rerunning same tests will remove the last pcap file for that test and start a new one.


### Generating the email template

To generate the email template, ensure that a report named `validation_test_report.xlsx` already exists in the source directory. This file is used as input to create a technician-ready HTML email preview.

The email includes:
- Metadata rows (e.g., technician name, firmware version)
- A formatted test result table with conditional coloring
- Additional sheets like `Technician_Issues` are rendered as simple tables
- Automatically attached images if the correct folder path exist

Once ready, run:

- .\vtg.exe --email-gen example.sender@example.com example.recipient@example.com


### Automated sensor system configuration dump

This tool can automatically do configuration dump from a powered sensor.

To bypass this functionality in the event of sensor not working, you can run the email
generator command with the bypass flag.

The sensor configuration dump with output to an email attachment directory.
