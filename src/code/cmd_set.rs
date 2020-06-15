use log::{error, trace};

use crate::{
    code::{config::Config, App},
    event::Event,
    Result,
};

#[derive(Clone, Default)]
pub struct Set {
    param: String,
}

impl Set {
    pub fn new(param: String) -> Self {
        Set { param }
    }
}

impl Set {
    pub fn on_command(&mut self, app: &mut App) -> Result<Event> {
        let config: &mut Config = app.as_mut();
        match self.param.as_str() {
            "wrap" => {
                config.wrap = true;
                trace!("set all windows to wrap text");
            }
            "nowrap" => {
                config.wrap = false;
                trace!("set all windows to non-wrap text");
            }
            "ro" => {
                config.read_only = true;
                trace!("set default file open to read-only mode");
            }
            "noro" => {
                config.read_only = false;
                trace!("set default file open to read-write mode");
            }
            _ => {
                error!("invalid configuration command {}", self.param);
            }
        }

        Ok(Event::Noop)
    }
}
