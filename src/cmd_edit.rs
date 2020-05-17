#[derive(Clone)]
pub enum Edit {
    Initial {
        name: String,
    },
    TabComp {
        name: String,
        choices: Vec<String>,
        choice: usize,
    },
}

impl Default for Edit {
    fn default() -> Edit {
        Edit::Initial {
            name: "edit".to_string(),
        }
    }
}

impl Edit {
    fn to_name(&self) -> String {
        match self {
            Edit::Initial { name } => name.clone(),
            Edit::TabComp { name, .. } => name.clone(),
        }
    }
}
