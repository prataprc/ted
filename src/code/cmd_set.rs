use crate::{code::App, Result};

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
    pub fn on_command(&mut self, app: &mut App) -> Result<()> {
        todo!()
    }
}
