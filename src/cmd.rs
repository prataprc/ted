#[derive(Clone)]
pub enum Command {
    None,
}

impl Default for Command {
    fn default() -> Command {
        Command::None
    }
}
