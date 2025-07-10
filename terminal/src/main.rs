use std::io::stdin;

use engine::SearchEngine;

use crate::processors::{process_command_line_args, MiscProcessor};

mod processors;

fn main() {
    let mut shutdown_token = false;

    let mut search_engine = SearchEngine::default();

    if process_command_line_args(&std::env::args().collect(), &search_engine) {
        return;
    }

    type CommandProcessorFunc = fn(&str, &[String], &mut SearchEngine, &mut bool) -> bool;
    const COMMAND_PROCESSORS: [CommandProcessorFunc; 1] = [MiscProcessor::execute];

    while !shutdown_token {
        let mut input_command = String::new();

        if stdin().read_line(&mut input_command).is_err() {
            println!("Error reading input, please try again.");
            continue;
        }

        let input_command = input_command.trim();
        let command_parts: Vec<&str> = input_command.split_whitespace().collect();
        if command_parts.is_empty() {
            continue;
        }

        let command = command_parts[0];
        let command_args = &command_parts[1..]
            .iter()
            .map(|&arg_str| arg_str.to_string())
            .collect::<Vec<String>>();

        for processor in &COMMAND_PROCESSORS {
            if processor(command, &command_args, &mut search_engine, &mut shutdown_token) {
                break;
            }
        }
    }
}
