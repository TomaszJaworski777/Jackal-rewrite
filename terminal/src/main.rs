use engine::SearchEngine;

use crate::{
    displays::welcome_message,
    processors::{process_command_line_args, MiscProcessor, UciProcessor},
};

mod displays;
mod input_wrapper;
mod processors;

pub use input_wrapper::InputWrapper;

fn main() {
    let mut shutdown_token = false;

    let mut search_engine = SearchEngine::new();

    if process_command_line_args(&std::env::args().collect(), &mut search_engine) {
        return;
    }

    println!("{}", welcome_message());

    let mut input_wrapper = InputWrapper::new();
    let mut uci_processor = UciProcessor::new();

    while !shutdown_token {
        let input_command = input_wrapper.get_input();

        let command_parts: Vec<&str> = input_command.split_whitespace().collect();
        if command_parts.is_empty() {
            continue;
        }

        let command = command_parts[0];
        let command_args = &command_parts[1..]
            .iter()
            .map(|&arg_str| arg_str.to_string())
            .collect::<Vec<String>>();

        if MiscProcessor::execute(
            command,
            &command_args,
            &mut search_engine,
            &mut shutdown_token,
        ) {
            continue;
        }

        if uci_processor.execute(
            command,
            &command_args,
            &mut search_engine,
            &mut input_wrapper,
            &mut shutdown_token,
        ) {
            continue;
        }
    }
}
