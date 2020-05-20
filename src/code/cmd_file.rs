#[derive(Clone)]
pub enum File {
    Initial {
        name: String,
    },
    TabComp {
        name: String,
        choices: Vec<String>,
        choice: usize,
    },
}

impl Default for File {
    fn default() -> File {
        File::Initial {
            name: "file".to_string(),
        }
    }
}

impl File {
    pub fn to_name(&self) -> String {
        match self {
            File::Initial { name } => name.clone(),
            File::TabComp { name, .. } => name.clone(),
        }
    }
}
