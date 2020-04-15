#[derive(Clone, Debug)]
pub struct Config {
    pub tabstop: u16,
    pub scroll_off: u16,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            tabstop: 4,
            scroll_off: 0,
        }
    }
}
