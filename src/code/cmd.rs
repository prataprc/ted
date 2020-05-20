use log::error;

use crate::{
    cmd_edit::Edit, cmd_file::File, cmd_write::Write, state::State, tabc::TabComplete,
    window::Window, Result,
};

pub enum Commands {
    Initial {
        cmds: Vec<Command>,
    },
    TabComp {
        tabc: TabComplete,
        cmds: Vec<Command>,
    },
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

impl Commands {
    fn to_choices(span: &str, cmds: &[Command]) -> Vec<String> {
        let iter = cmds.iter().filter_map(|o| {
            let name = o.to_name();
            if_else!(name.starts_with(span), Some(name), None)
        });
        iter.collect()
    }

    fn to_command_name(s: &mut State) -> String {
        //let s = c.as_buffer().to_string();
        //let parts: Vec<&str> = s.split(' ').collect();
        //match parts.as_slice() {
        //    [name] => name.to_string(),
        //    [name, ..] => name.to_string(),
        //    [] => "".to_string(),
        //}
        todo!()
    }
}

impl Commands {
    pub fn on_tab(&mut self, s: &mut State) -> Result<()> {
        let span = Self::to_command_name(s);

        match self {
            Commands::Initial { cmds } => {
                let tabc = {
                    let choices = Self::to_choices(&span, &cmds);
                    TabComplete::new(span, choices)
                };
                let cmds: Vec<Command> = cmds.drain(..).collect();
                *self = Commands::TabComp { tabc, cmds };
            }
            Commands::TabComp { tabc, .. } if tabc.is_same(&span) => (),
            Commands::TabComp { tabc: _, cmds } => {
                let tabc = {
                    let choices = Self::to_choices(&span, cmds);
                    TabComplete::new(span, choices)
                };
                let cmds: Vec<Command> = cmds.drain(..).collect();
                *self = Commands::TabComp { tabc, cmds };
            }
        }

        match self {
            Commands::TabComp { tabc, .. } => {
                //use crate::window_code::Message;

                //let w = match c.to_window() {
                //    Window::Code(mut w) => {
                //        w.post(c, Message::TabComplete(tabc.clone()))?;
                //        Window::Code(w)
                //    }
                //    w => w,
                //};
                //c.set_window(w);
            }
            Commands::Initial { .. } => error!("unreachable"),
        }

        Ok(())
    }

    pub fn on_command(&mut self, _: &mut State) -> Result<()> {
        todo!()
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

impl Command {
    fn to_name(&self) -> String {
        match self {
            Command::Edit(o) => o.to_name(),
            Command::File(o) => o.to_name(),
            Command::Write(o) => o.to_name(),
            Command::None => unreachable!(),
        }
    }
}
