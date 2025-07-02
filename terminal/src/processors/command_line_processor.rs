const DEFAULT_BENCH_DEPTH: usize = 5;

#[allow(clippy::ptr_arg)]
pub fn process_command_line_args(args: &Vec<String>) -> bool {
    let mut result = false;

    for (idx, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "bench" => {
                let depth = if idx >= args.len() - 1 {
                    DEFAULT_BENCH_DEPTH
                } else {
                    args[idx + 1]
                        .parse::<usize>()
                        .unwrap_or(DEFAULT_BENCH_DEPTH)
                };

                //TODO: Call bench
                result = true;
            }
            _ => continue,
        }
    }

    result
}
