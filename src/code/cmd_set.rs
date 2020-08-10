#[allow(unused_imports)]
use log::{debug, error, trace};

use crate::{code::cmd::Command, code::Code, event::Event, syntax, Result};

pub struct Set {
    inner: Inner,
}

enum Inner {
    Config { param: String },
    None,
}

impl Set {
    pub fn new(syn: syntax::CodeCmd) -> Result<Self> {
        match Self::convert(syn) {
            Some(inner) => Ok(Set { inner }),
            None => Ok(Set { inner: Inner::None }),
        }
    }

    fn convert(syn: syntax::CodeCmd) -> Option<Inner> {
        let tree = syn.into_parse_tree()?;

        let node_cmd = {
            let root = tree.root_node();
            root.child(root.child_count().saturating_sub(1))?
        };

        let node_set = node_cmd.child(0)?;
        assert_eq!(node_set.kind(), "set", "{}", node_set.kind());

        let node_config_param = node_set.child(1)?;
        let param = node_config_param.child(0)?.kind().to_string();
        Some(Inner::Config { param })
    }
}

impl Command for Set {
    fn on_command(&mut self, _app: &mut Code) -> Result<Event> {
        // use crate::code::config::Config
        //let config: &mut Config = app.as_mut();
        //match self.args.as_str() {
        //    "wrap" => {
        //        config.wrap = true;
        //        debug!("set all windows to wrap text");
        //    }
        //    "nowrap" => {
        //        config.wrap = false;
        //        debug!("set all windows to non-wrap text");
        //    }
        //    "ro" => {
        //        config.read_only = true;
        //        debug!("set default file open to read-only mode");
        //    }
        //    "noro" => {
        //        config.read_only = false;
        //        debug!("set default file open to read-write mode");
        //    }
        //    _ => {
        //        error!("invalid configuration command {}", self.args);
        //    }
        //}

        Ok(Event::Noop)
    }
}
