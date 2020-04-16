#[derive(Clone, Debug)]
pub struct Config {
    pub tabstop: String,
    pub scroll_off: u16,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            tabstop: "    ".to_string(),
            scroll_off: 0,
        }
    }
}
