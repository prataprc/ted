use crate::{cmd_edit::Edit, cmd_file::File, cmd_write::Write};

pub enum Commands {
    Initial { cmds: Vec<Command> },
    TabComp { cmd: Command, cmds: Vec<Command> },
}

impl Default for Commands {
    fn default() -> Commands {
        let cmds = vec![
            Command::Edit(Default::default()),
            Command::File(Default::default()),
            Command::Write(Default::default()),
        ];
        Commands::Initial { cmds }
    }
}

#[derive(Clone)]
pub enum Command {
    Edit(Edit),
    File(File),
    Write(Write),
    None,
}

impl Default for Command {
    fn default() -> Command {
        Command::None
    }
}
