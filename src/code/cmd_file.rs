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
