#[derive(Clone, Debug)]
pub struct Config {
    pub tabstop: String,
    pub scroll_off: u16,
    pub left_margin_char: char,
    pub top_margin_char: char,
    pub line_number: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            tabstop: "    ".to_string(),
            scroll_off: 0,
            left_margin_char: '|',
            top_margin_char: '-',
            line_number: true,
        }
    }
}
