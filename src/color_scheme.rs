use crossterm::style::{Attribute, Color};
#[allow(unused_imports)]
use log::trace;
use toml;

use std::{convert::TryFrom, fmt, iter::FromIterator, result};

use crate::{Error, Result};

/// Colorscheme for ted applications.
#[derive(Debug)]
pub struct ColorScheme {
    name: String,
    hs: Vec<Style>,
}

impl Default for ColorScheme {
    fn default() -> ColorScheme {
        let toml_style: toml::Value = DEFAULT_STYLE.parse().unwrap();
        TryFrom::try_from(toml_style).unwrap()
    }
}

impl TryFrom<toml::Value> for ColorScheme {
    type Error = Error;

    fn try_from(value: toml::Value) -> Result<Self> {
        use crate::Error::Invalid;

        let table = {
            let err = Invalid(format!("bad color sheme"));
            value.as_table().ok_or(err)?
        };

        let mut name = Default::default();
        let mut hs: Vec<Style> = {
            let mut hs = Vec::with_capacity(Highlight::__Fin as usize);
            hs.resize(hs.capacity(), Default::default());
            hs
        };

        for (key, value) in table.iter() {
            match key.as_str() {
                "name" => {
                    let err = Invalid(format!("bad value for {}", key));
                    name = value.as_str().ok_or(err)?.to_string();
                }
                hl => {
                    let off = {
                        let h: Highlight = TryFrom::try_from(hl)?;
                        (h as u32) as usize
                    };
                    hs[off] = TryFrom::try_from(value.clone())?;
                    trace!("convert {} {} {:?}", hl, off, hs[off]);
                }
            }
        }

        Ok(ColorScheme { name, hs })
    }
}

impl ColorScheme {
    pub fn to_style(&self, hl: Highlight) -> Style {
        self.hs[hl as u32 as usize].clone()
    }
}

const DEFAULT_STYLE: &'static str = r##"
name            = "default"
canvas          = { on = "#373737", with = "#b9b9b9"}
comment         = { on = "#373737", with = "#4e4e4e"}
constant        = { on = "#373737", with = "#4e4e4e"}
string          = { on = "#373737", with = "#4e4e4e"}
char            = { on = "#373737", with = "#4e4e4e"}
number          = { on = "#373737", with = "#4e4e4e"}
boolean         = { on = "#373737", with = "#4e4e4e"}
float           = { on = "#373737", with = "#4e4e4e"}
identifier      = { on = "#373737", with = "#4e4e4e"}
function        = { on = "#373737", with = "#4e4e4e"}
statement       = { on = "#373737", with = "#4e4e4e"}
conditional     = { on = "#373737", with = "#4e4e4e"}
repeat          = { on = "#373737", with = "#4e4e4e"}
label           = { on = "#373737", with = "#4e4e4e"}
operator        = { on = "#373737", with = "#4e4e4e"}
keyword         = { on = "#373737", with = "#4e4e4e"}
exception       = { on = "#373737", with = "#4e4e4e"}
preproc         = { on = "#373737", with = "#4e4e4e"}
include         = { on = "#373737", with = "#4e4e4e"}
define          = { on = "#373737", with = "#4e4e4e"}
macro           = { on = "#373737", with = "#4e4e4e"}
precondit       = { on = "#373737", with = "#4e4e4e"}
type            = { on = "#373737", with = "#4e4e4e"}
storage-class   = { on = "#373737", with = "#4e4e4e"}
structure       = { on = "#373737", with = "#4e4e4e"}
typedef         = { on = "#373737", with = "#4e4e4e"}
special         = { on = "#373737", with = "#4e4e4e"}
special-char    = { on = "#373737", with = "#4e4e4e"}
tag             = { on = "#373737", with = "#4e4e4e"}
delimiter       = { on = "#373737", with = "#4e4e4e"}
special-comment = { on = "#373737", with = "#4e4e4e"}
debug           = { on = "#373737", with = "#4e4e4e"}
underline       = { on = "#373737", with = "#4e4e4e"}
ignore          = { on = "#373737", with = "#4e4e4e"}
error           = { on = "#373737", with = "#e64f4f"}
todo            = { on = "#373737", with = "#4e4e4e"}
line-nr         = { on = "#444444", with = "#86875f"}
prompt          = { on = "#373737", with = "#cf7d00"}
status-line     = { on = "#1c2c1c", with = "#cf7d00"}
tab-line        = { on = "#272727", with = "#cf7d00"}
tab-option      = { on = "#272727", with = "#cf7d00"}
tab-select      = { on = "#272727", with = "#cf7d00"}
"##;

#[derive(Clone)]
pub struct Style {
    pub bg: Color,
    pub fg: Color,
    pub attrs: Vec<Attribute>,
}

impl fmt::Debug for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "Style<{:?},{:?}>", self.bg, self.fg)
    }
}

impl Default for Style {
    fn default() -> Style {
        Style {
            fg: Color::White,
            bg: Color::Black,
            attrs: Default::default(),
        }
    }
}

impl TryFrom<toml::Value> for Style {
    type Error = Error;

    fn try_from(value: toml::Value) -> Result<Self> {
        use crate::Error::Invalid;

        let table = {
            let err = Invalid(format!("bad style"));
            value.as_table().ok_or(err)?
        };

        let mut style: Style = Default::default();
        for (key, value) in table.iter() {
            let value = {
                let msg = format!("bad style key:{:?} value:{:?}", key, value);
                value.as_str().ok_or(Invalid(msg))?
            };
            match key.as_str() {
                "on" | "bg" => style.bg = Style::to_color(value)?,
                "with" | "fg" => style.fg = Style::to_color(value)?,
                "attr" | "attribute" => style.attrs = Style::to_attrs(value)?,
                _ => (),
            }
        }

        Ok(style)
    }
}

impl Style {
    fn to_color(color: &str) -> Result<Color> {
        use std::iter::repeat;

        let n = color.len();
        let color = match color {
            "reset" => Color::Reset,
            "black" => Color::Black,
            "darkgrey" | "dark-grey" | "dark_grey" => Color::DarkGrey,
            "red" => Color::Red,
            "darkred" | "dark-red" | "dark_red" => Color::DarkRed,
            "green" => Color::Green,
            "darkgreen" | "dark-green" | "dark_green" => Color::DarkGreen,
            "yellow" => Color::Yellow,
            "darkyellow" | "dark-yellow" | "dark_yellow" => Color::DarkYellow,
            "blue" => Color::Blue,
            "darkblue" | "dark-blue" | "dark_blue" => Color::DarkBlue,
            "magenta" => Color::Magenta,
            "darkmagenta" | "dark-magenta" | "dark_magenta" => Color::DarkMagenta,
            "cyan" => Color::Cyan,
            "darkcyan" | "dark-cyan" | "dark_cyan" => Color::DarkCyan,
            "white" => Color::White,
            "grey" => Color::Grey,
            _ if n == 0 => Color::Rgb { r: 0, g: 0, b: 0 },
            color => match color.chars().next() {
                Some('#') if n == 1 => Color::Rgb { r: 0, g: 0, b: 0 },
                Some('#') => {
                    let from_str_radix = u8::from_str_radix;

                    let p = {
                        let iter = repeat('0').take(6_usize.saturating_sub(n));
                        String::from_iter(iter)
                    };
                    let s = p + &color[1..];
                    let r = err_at!(FailConvert, from_str_radix(&s[0..2], 16))?;
                    let g = err_at!(FailConvert, from_str_radix(&s[2..4], 16))?;
                    let b = err_at!(FailConvert, from_str_radix(&s[4..6], 16))?;
                    Color::Rgb { r, g, b }
                }
                Some(_) => {
                    let n: u8 = err_at!(FailConvert, color.parse())?;
                    Color::AnsiValue(n)
                }
                None => err_at!(FailConvert, msg: format!("invalid color"))?,
            },
        };

        Ok(color)
    }

    fn to_attrs(attr: &str) -> Result<Vec<Attribute>> {
        let ss: Vec<&str> = if attr.contains(",") {
            attr.split(",").collect()
        } else if attr.contains("|") {
            attr.split("|").collect()
        } else {
            vec![attr]
        };

        let mut attrs: Vec<Attribute> = Default::default();
        for item in ss.into_iter() {
            match item {
                "bold" => attrs.push(Attribute::Bold),
                "italic" => attrs.push(Attribute::Italic),
                "underlined" => attrs.push(Attribute::Underlined),
                "underline" => attrs.push(Attribute::Underlined),
                "reverse" => attrs.push(Attribute::Reverse),
                _ => err_at!(Invalid, msg: format!("invalid attr {:?}", item))?,
            }
        }
        Ok(attrs)
    }
}

macro_rules! highlight {
    ($(($variant:ident, $s:expr)),*) => (
        #[derive(Clone, Copy)]
        pub enum Highlight {
            Canvas = 0,
            $($variant,)*
        }

        impl Default for Highlight {
            fn default() -> Self {
                Highlight::Canvas
            }
        }

        impl<'a> TryFrom<&'a str> for Highlight {
            type Error = Error;

            fn try_from(s: &'a str) -> Result<Highlight> {
                match s {
                    $($s => Ok(Highlight::$variant),)*
                    "canvas" => Ok(Highlight::Canvas),
                    _ => {
                        let msg = format!("invalid highlight {}", s);
                        err_at!(FailConvert, msg: msg)
                    }
                }
            }
        }

        impl fmt::Display for Highlight {
            fn fmt(
                //
                &self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
                match self {
                    Highlight::Canvas => write!(f, "canvas"),
                    $(Highlight::$variant => write!(f, $s),)*
                }
            }
        }
    );
}

highlight![
    // code syntax.
    (Comment, "comment"),
    (Constant, "constant"),
    (Str, "string"),
    (Character, "char"),
    (Number, "number"),
    (Boolean, "boolean"),
    (Float, "float"),
    (Identifier, "identifier"),
    (Function, "function"),
    (Statement, "statement"),
    (Conditional, "conditional"),
    (Repeat, "repeat"),
    (Label, "label"),
    (Operator, "operator"),
    (Keyword, "keyword"),
    (Exception, "exception"),
    (PreProc, "preproc"),
    (Include, "include"),
    (Define, "define"),
    (Macro, "macro"),
    (PreCondit, "precondit"),
    (Type, "type"),
    (StorageClass, "storage-class"),
    (Structure, "structure"),
    (Typedef, "typedef"),
    (Special, "special"),
    (SpecialChar, "special-char"),
    (Tag, "tag"),
    (Delimiter, "delimiter"),
    (SpecialComment, "special-comment"),
    (Debug, "debug"),
    (Underline, "underline"),
    (Ignore, "ignore"),
    (Error, "error"),
    (Todo, "todo"),
    // system highlight
    (LineNr, "line-nr"),
    (Prompt, "prompt"),
    (StatusLine, "status-line"),
    (TabLine, "tab-line"),
    (TabOption, "tab-option"),
    (TabSelect, "tab-select"),
    //ColorColumn
    //Conceal
    //Cursor
    //CursorColumn
    //CursorIM
    //CursorLine
    //CursorLineNr
    //DiffAdd
    //DiffChange
    //DiffDelete
    //DiffText
    //Directory
    //EndOfBuffer
    //ErrorMsg
    //FoldColumn
    //Folded
    //IncSearch
    //lCursor
    //LineNrAbove
    //LineNrBelow
    //MatchParen
    //Menu
    //ModeMsg
    //MoreMsg
    //NonText
    //Pmenu
    //PmenuSbar
    //PmenuSel
    //PmenuThumb
    //Question
    //QuickFixLine
    //Scrollbar
    //Search
    //SignColumn
    //SpecialKey
    //SpellBad
    //SpellCap
    //SpellLocal
    //SpellRare
    //StatusLineNC
    //StatusLineTerm
    //StatusLineTermNC
    //Terminal
    //Title
    //Tooltip
    //VertSplit
    //Visual
    //VisualNOS
    //WarningMsg
    //WildMenu
    (__Fin, "__fin")
];
