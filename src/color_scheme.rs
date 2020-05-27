use crossterm::style::{Attribute, Color};
use toml;

use std::{convert::TryFrom, fmt, iter::FromIterator, result};

use crate::{Error, Result};

/// Colorscheme for ted applications.
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
                        let h: Highlight = hl.into();
                        (h as u32) as usize
                    };
                    hs[off] = TryFrom::try_from(value.clone())?;
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
canvas          = { on = "#3a3a3a", with = "#4e4e4e"}
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
error           = { on = "#373737", with = "#4e4e4e"}
todo            = { on = "#373737", with = "#4e4e4e"}
line-nr         = { on = "#444444", with = "#86875f"}
"##;

#[derive(Clone)]
pub struct Style {
    pub fg: Color,
    pub bg: Color,
    pub attrs: Vec<Attribute>,
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

#[derive(Clone, Copy)]
pub enum Highlight {
    Canvas = 0,
    // code syntax.
    Comment,
    Constant,
    Str,
    Character,
    Number,
    Boolean,
    Float,
    Identifier,
    Function,
    Statement,
    Conditional,
    Repeat,
    Label,
    Operator,
    Keyword,
    Exception,
    PreProc,
    Include,
    Define,
    Macro,
    PreCondit,
    Type,
    StorageClass,
    Structure,
    Typedef,
    Special,
    SpecialChar,
    Tag,
    Delimiter,
    SpecialComment,
    Debug,
    Underlined,
    Ignore,
    Error,
    Todo,

    // system highlight
    LineNr,
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
    //StatusLine
    //StatusLineNC
    //StatusLineTerm
    //StatusLineTermNC
    //TabLine
    //TabLineFill
    //TabLineSel
    //Terminal
    //Title
    //Tooltip
    //VertSplit
    //Visual
    //VisualNOS
    //WarningMsg
    //WildMenu
    __Fin,
}

impl Default for Highlight {
    fn default() -> Highlight {
        Highlight::Canvas
    }
}

impl<'a> From<&'a str> for Highlight {
    fn from(s: &'a str) -> Highlight {
        match s {
            "canvas" => Highlight::Canvas,
            //
            "comment" => Highlight::Comment,
            "constant" => Highlight::Constant,
            "string" => Highlight::Str,
            "char" => Highlight::Character,
            "number" => Highlight::Number,
            "boolean" => Highlight::Boolean,
            "float" => Highlight::Float,
            "identifier" => Highlight::Identifier,
            "function" => Highlight::Function,
            "statement" => Highlight::Statement,
            "conditional" => Highlight::Conditional,
            "repeat" => Highlight::Repeat,
            "label" => Highlight::Label,
            "operator" => Highlight::Operator,
            "keyword" => Highlight::Keyword,
            "exception" => Highlight::Exception,
            "preproc" => Highlight::PreProc,
            "include" => Highlight::Include,
            "define" => Highlight::Define,
            "macro" => Highlight::Macro,
            "precondit" => Highlight::PreCondit,
            "type" => Highlight::Type,
            "storage-class" => Highlight::StorageClass,
            "structure" => Highlight::Structure,
            "typedef" => Highlight::Typedef,
            "special" => Highlight::Special,
            "special-char" => Highlight::SpecialChar,
            "tag" => Highlight::Tag,
            "delimiter" => Highlight::Delimiter,
            "special-comment" => Highlight::SpecialComment,
            "debug" => Highlight::Debug,
            "underline" => Highlight::Underlined,
            "ignore" => Highlight::Ignore,
            "error" => Highlight::Error,
            "todo" => Highlight::Todo,
            // system highlight
            "line-nr" => Highlight::LineNr,
            _ => Highlight::Canvas,
        }
    }
}

impl fmt::Display for Highlight {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
            Highlight::Canvas => write!(f, "canvas"),
            //
            Highlight::Comment => write!(f, "comment"),
            Highlight::Constant => write!(f, "constant"),
            Highlight::Str => write!(f, "string"),
            Highlight::Character => write!(f, "char"),
            Highlight::Number => write!(f, "number"),
            Highlight::Boolean => write!(f, "boolean"),
            Highlight::Float => write!(f, "float"),
            Highlight::Identifier => write!(f, "identifier"),
            Highlight::Function => write!(f, "function"),
            Highlight::Statement => write!(f, "statement"),
            Highlight::Conditional => write!(f, "conditional"),
            Highlight::Repeat => write!(f, "repeat"),
            Highlight::Label => write!(f, "label"),
            Highlight::Operator => write!(f, "operator"),
            Highlight::Keyword => write!(f, "keyword"),
            Highlight::Exception => write!(f, "exception"),
            Highlight::PreProc => write!(f, "preproc"),
            Highlight::Include => write!(f, "include"),
            Highlight::Define => write!(f, "define"),
            Highlight::Macro => write!(f, "macro"),
            Highlight::PreCondit => write!(f, "precondit"),
            Highlight::Type => write!(f, "type"),
            Highlight::StorageClass => write!(f, "storage-class"),
            Highlight::Structure => write!(f, "structure"),
            Highlight::Typedef => write!(f, "typedef"),
            Highlight::Special => write!(f, "special"),
            Highlight::SpecialChar => write!(f, "special-char"),
            Highlight::Tag => write!(f, "tag"),
            Highlight::Delimiter => write!(f, "delimiter"),
            Highlight::SpecialComment => write!(f, "special-comment"),
            Highlight::Debug => write!(f, "debug"),
            Highlight::Underlined => write!(f, "underline"),
            Highlight::Ignore => write!(f, "ignore"),
            Highlight::Error => write!(f, "error"),
            Highlight::Todo => write!(f, "todo"),
            // system highlight
            Highlight::LineNr => write!(f, "line-nr"),
            Highlight::__Fin => unreachable!(),
        }
    }
}
