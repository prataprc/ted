#[derive(Clone)]
struct Config {
    tabstop: u16,
}

impl Default for Config {
    fn default() -> Config {
        Config { tabstop: 4 }
    }
}
