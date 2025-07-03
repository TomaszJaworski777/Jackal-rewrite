use utils::clear_terminal_screen;

pub struct MiscProcessor;
impl MiscProcessor {
    pub fn execute(command: &str, args: &[String], cancelation_token: &mut bool) -> bool {
        match command {
            "exit" | "quit" | "q" => *cancelation_token = true,
            "clear" | "clean" | "cls" => clear_terminal_screen(),
            _ => return false,
        }

        true
    }
}
