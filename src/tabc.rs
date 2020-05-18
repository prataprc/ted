#[derive(Clone)]
pub struct TabComplete {
    span: String,
    choices: Vec<String>,
    choice: Option<usize>,
}

impl TabComplete {
    pub fn new(span: String, choices: Vec<String>) -> TabComplete {
        TabComplete {
            span,
            choices,
            choice: None,
        }
    }

    pub fn is_same(&self, span: &str) -> bool {
        self.span == span
    }
}

impl Iterator for TabComplete {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let n_choices = self.choices.len();
        self.choice = match &self.choice {
            Some(choice) if (choice + 1) == n_choices => None,
            Some(choice) => Some(choice + 1),
            None if n_choices == 0 => None,
            None => Some(0),
        };
        match &self.choice {
            Some(choice) => Ok(self.choices[choice].clone()),
            None => Ok(self.span.clone()),
        }
    }
}
