use engine::SearchEngine;

pub struct UciProcessor;
impl UciProcessor {
    pub fn execute(command: &str, args: &[String], search_engine: &mut SearchEngine, shutdown_token: &mut bool) -> bool {
        match command {
            _ => return false,
        }

        true
    }
}