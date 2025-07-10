use engine::SearchEngine;

#[allow(clippy::ptr_arg)]
pub fn process_command_line_args(args: &Vec<String>, search_engine: &SearchEngine) -> bool {
    let mut result = false;

    for (idx, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "bench" => {
                let depth = if idx >= args.len() - 1 {
                    None
                } else {
                    args[idx + 1].parse::<u8>().ok()
                };

                search_engine.bench(depth);
                result = true;
            }
            _ => continue,
        }
    }

    result
}
