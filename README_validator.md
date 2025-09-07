## VALIDATION REPORT GENERATION - validator

### Features
- Protection against script files tampering
- protection against grouped test toml tampering
- Generate a technician-friendly Excel report, Includes metadata rows for technician and firmware details
- Validation test runner
- Validation email template generator
- Capture pcap files from test runs
- Automated sensor system configuration dump

### Using the tool
You can run the tool directly using .\vtg.exe.

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
