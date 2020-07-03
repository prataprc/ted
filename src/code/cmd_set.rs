#[allow(unused_imports)]
use log::{debug, error, trace};

use crate::{
    code::cmd::Command,
    code::{config::Config, Code},
    event::Event,
    syntax, Result,
};

#[derive(Clone)]
pub struct Set {
    line: String,
    syn: syntax::CodeCmd,
}

impl Set {
    pub fn new(line: String, syn: syntax::CodeCmd) -> Self {
        Set { line, syn }
    }
}

impl Command for Set {
    fn on_command(&mut self, app: &mut Code) -> Result<Event> {
        let config: &mut Config = app.as_mut();
        match self.args.as_str() {
            "wrap" => {
                config.wrap = true;
                debug!("set all windows to wrap text");
            }
            "nowrap" => {
                config.wrap = false;
                debug!("set all windows to non-wrap text");
            }
            "ro" => {
                config.read_only = true;
                debug!("set default file open to read-only mode");
            }
            "noro" => {
                config.read_only = false;
                debug!("set default file open to read-write mode");
            }
            _ => {
                error!("invalid configuration command {}", self.args);
            }
        }

        Ok(Event::Noop)
    }
}
