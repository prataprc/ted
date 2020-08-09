use lazy_static::lazy_static;
#[allow(unused_imports)]
use log::{debug, trace};

use std::convert::TryFrom;

use crate::{
    code::{cmd_set::Set, Code},
    colors::ColorScheme,
    event::Event,
    syntax, Error, Result,
};

pub trait Command {
    fn on_command(&mut self, app: &mut Code) -> Result<Event>;
}

macro_rules! commands {
    ($(($var:ident, $t:ident, $name:expr)),*) => (
        lazy_static! {
            static ref CMD_NAMES: Vec<String> = vec![
                $($name.to_string())*
            ];
        }

        pub enum Cmd {
            $($var($t),)*
        }

        // generate the cmd from (:full-cmd-line, color-scheme)
        impl TryFrom<(String, ColorScheme)> for Cmd {
            type Error = Error;

            fn try_from((line, scheme): (String, ColorScheme)) -> Result<Self> {
                let syn = syntax::CodeCmd::new(&line, scheme.clone())?;
                let err = Error::Invalid("".to_string(), format!("no command"));
                let name = err_at!(syn.to_command_name().ok_or(err))?;
                match name.as_str() {
                    $($name => Ok(Cmd::$var($t::new(syn, scheme)?)),)*
                    name => err_at!(Invalid, msg: format!("command {}", name)),
                }
            }
        }

        impl Command for Cmd {
            fn on_command(&mut self, app: &mut Code) -> Result<Event> {
                match self {
                    $(Cmd::$var(val) => val.on_command(app),)*
                }
            }
        }
    )
}

commands![(Set, Set, "set")];

//pub fn on_tab(&mut self, s: &mut State) -> Result<()> {
//    let span = Self::to_command_name(s);

//    match self {
//        Cmd::Initial { cmds } => {
//            let tabc = {
//                let choices = Self::to_cmd_names(&span, &cmds);
//                TabComplete::new(span, choices)
//            };
//            let cmds: Vec<Cmd> = cmds.drain(..).collect();
//            *self = Cmd::TabComp { tabc, cmds };
//        }
//        Cmd::TabComp { tabc, .. } if tabc.is_same(&span) => (),
//        Cmd::TabComp { tabc: _, cmds } => {
//            let tabc = {
//                let choices = Self::to_cmd_names(&span, cmds);
//                TabComplete::new(span, choices)
//            };
//            let cmds: Vec<Cmd> = cmds.drain(..).collect();
//            *self = Cmd::TabComp { tabc, cmds };
//        }
//    }

//    match self {
//        Cmd::TabComp { tabc, .. } => {
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
//        Cmd::Initial { .. } => error!("unreachable"),
//    }

//    Ok(())
//}
