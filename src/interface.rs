use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "vtg",
    version = "2.1",
    about = "Generates filtered validation test reports"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Generate email report
    EmailGen {
        #[arg(value_name = "SENDER_EMAIL")]
        sender_email: String,
        #[arg(value_name = "RECIPIENT_EMAIL")]
        recipient_email: String,
        #[arg(short = 'g', long = "generate", help = "Generate email attachments")]
        generate: bool,
    },
    #[command(
        about = "Run specific test instructions",
        long_about = r#"

Run specific test instructions.

- Option to specify the test Id(s)
    vtg test 1.1
    vtg test 1.1 1.2 1.3

- Option to specify the input test file.
No specified input file will use the default instruction file name in the same
directory as the executable.
    vtg test 1.1 -i Path/To/Instruction_FIle
    vtg test 1.1 1.2 1.3 -i Path/To/Instruction_FIle

- No input Id's will print all the tests in the test file and prompt to run all tests:
    vtg test
    vtg test -i Path/To/Instruction_File
"#
    )]
    Test {
        #[arg(value_name = "TEST_ID")]
        test_ids: Option<Vec<String>>,
        #[arg(short = 'i', long = "input-instruction-file")]
        input_instruction_file: Option<String>,
    },
    /// Generate Excel report from grouped CSV
    Excel {
        #[arg(short = 'i', long = "input-instruction-file")]
        input_instruction_file: Option<String>,
    },
    #[command(
        about = "Group tests by label and IDs",
        long_about = r#"
Group tests by label and IDs.
- Grouping tests
    - Single
    vtg id-groups GROUP_LABEL_A:1.1,1.2,1.3
    - Multi
    vtg id-groups GROUP_LABEL_A:1.1,1.2,1.3 GROUP_LABEL_B:5.1,6.2,7.3

- Option to specify a priority
Will filter the Ids entered for the priority specified
    vtg id-groups GROUP_LABEL_A:1.1,1.2,1.3 GROUP_LABEL_B:5.1,6.2,7.3 -p HIGH

- Option to specify the input test file.
No specified input file will use the default base file name in the same
directory as the executable.
    vtg id-groups GROUP_LABEL_A:1.1,1.2,1.3 -i Path/To/Base_File

- Option to specify the output file name.
No specified input file will use the default output name in the same
directory as the executable. The name should not have the file type in it.
    vtg id-groups GROUP_LABEL_A:1.1,1.2,1.3 -o my_custom_list
"#
    )]
    IdGroups {
        #[arg(value_name = "LABEL:IDS", num_args = 1..)]
        groups: Vec<String>,
        #[arg(short = 'p', long = "priority")]
        priority: Option<String>,
        #[arg(short = 'i', long = "input-base-file")]
        input_instruction_file: Option<String>,
        #[arg(short = 'o', long = "output-name")]
        output_name: Option<String>,
    },
    /// Group tests by priority only
    Priority {
        #[arg(value_name = "PRIORITY")]
        priority: String,
        #[arg(short = 'i', long = "input-base-file")]
        input_instruction_file: Option<String>,
        #[arg(short = 'o', long = "output-name")]
        output_name: Option<String>,
    },
}
