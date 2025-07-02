use std::io::stdin;

use crate::processors::{process_command_line_args, MiscProcessor};

mod processors;

fn main() {
    //Create cancelation token used to kill the command loop
    let mut cancelation_token = false;

    //Process command line parameters
    if process_command_line_args(&std::env::args().collect()) {
        return;
    }

    //Register all command processors
    type CommandProcessorFunc = fn(&str, &[String], &mut bool) -> bool;
    const COMMAND_PROCESSORS: [CommandProcessorFunc; 1] =
        [MiscProcessor::execute];

    //Initialize engine loop
    while !cancelation_token {
        let mut input_command = String::new();

        //Read input from the console
        if stdin().read_line(&mut input_command).is_err() {
            println!("Error reading input, please try again.");
            continue;
        }

        //Separate command parts and skip if the command is empty
        let input_command = input_command.trim();
        let command_parts: Vec<&str> = input_command.split_whitespace().collect();
        if command_parts.is_empty() {
            continue;
        }

        //Construct command and it's arguments from input
        let command = command_parts[0];
        let command_args = &command_parts[1..]
            .iter()
            .map(|&arg_str| arg_str.to_string())
            .collect::<Vec<String>>();

        //Pass command through command processors and stop at the first processor that accepts this command
        for processor in &COMMAND_PROCESSORS {
            if processor(command, &command_args, &mut cancelation_token) {
                break;
            }
        }
    }
}
