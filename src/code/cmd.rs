#[allow(unused_imports)]
use log::trace;

use crate::{
    code::{cmd_set::Set, Code},
    event::Event,
    Result,
};

#[derive(Clone)]
pub enum Command {
    Set(Set),
    None,
}

impl Default for Command {
    fn default() -> Command {
        Command::None
    }
}

impl From<(String, String)> for Command {
    fn from(parts: (String, String)) -> Command {
        match parts.0.as_str() {
            ":set" => Command::Set(Set::new(parts.1)),
            _ => Default::default(),
        }
    }
}

impl Command {
    fn to_cmd_names(prefix: &str, cmds: &[Command]) -> Vec<String> {
        let iter = cmds.iter().filter_map(|o| {
            let name = o.to_name();
            if_else!(name.starts_with(prefix), Some(name), None)
        });
        iter.collect()
    }
}

impl Command {
    fn to_name(&self) -> String {
        match self {
            Command::Set(_) => "set".to_string(),
            Command::None => "invalid-command".to_string(),
        }
    }

    pub fn on_command(&mut self, app: &mut Code) -> Result<Event> {
        match self {
            Command::Set(val) => val.on_command(app),
            Command::None => Ok(Event::Noop),
        }
    }

    //pub fn on_tab(&mut self, s: &mut State) -> Result<()> {
    //    let span = Self::to_command_name(s);

    //    match self {
    //        Command::Initial { cmds } => {
    //            let tabc = {
    //                let choices = Self::to_cmd_names(&span, &cmds);
    //                TabComplete::new(span, choices)
    //            };
    //            let cmds: Vec<Command> = cmds.drain(..).collect();
    //            *self = Command::TabComp { tabc, cmds };
    //        }
    //        Command::TabComp { tabc, .. } if tabc.is_same(&span) => (),
    //        Command::TabComp { tabc: _, cmds } => {
    //            let tabc = {
    //                let choices = Self::to_cmd_names(&span, cmds);
    //                TabComplete::new(span, choices)
    //            };
    //            let cmds: Vec<Command> = cmds.drain(..).collect();
    //            *self = Command::TabComp { tabc, cmds };
    //        }
    //    }

    //    match self {
    //        Command::TabComp { tabc, .. } => {
    //            //use crate::window_code::Message;

    //            //let w = match c.to_window() {
    //            //    Window::Code(mut w) => {
    //            //        w.post(c, Message::TabComplete(tabc.clone()))?;
    //            //        Window::Code(w)
    //            //    }
    //            //    w => w,
    //            //};
    //            //c.set_window(w);
    //        }
    //        Command::Initial { .. } => error!("unreachable"),
    //    }

    //    Ok(())
    //}
}
