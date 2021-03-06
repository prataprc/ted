#[derive(Clone)]
pub enum Write {
    Initial {
        name: String,
    },
    TabComp {
        name: String,
        choices: Vec<String>,
        choice: usize,
    },
}

impl Default for Write {
    fn default() -> Write {
        Write::Initial {
            name: "write".to_string(),
        }
    }
}
