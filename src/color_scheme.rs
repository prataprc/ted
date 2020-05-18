use crossterm::style::{Attribute, Color},
use toml;

use crate::{Error, Result};

struct ColorScheme {
    name: String,
    hs: Vec<Style>,
}

impl FromStr for ColorScheme {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        use crate::Error::Invalid;

        let table = {
            let value: toml::Value = err_at!(FailConvert, s.parse())?;
            value.as_table().ok_or(Err(Invalid(format!("bad color sheme"))))?;
        };

        let mut name = Default::default();
        let mut hs = {
            let hs = Vec::with_capacity(Highlight::__Fin as usize);
            hs.resize(hs.capacity(), Highlight::__Fin);
            hs
        };
        for (key, value) in table.iter() {
            match key {
                "name" => {
                    let err = Err(Invalid(format!("bad value for {}", key)));
                    name = value.as_str().ok_or(err)?.to_string();
                }
                hl => {
                    let off = {
                        let h: Highlight = hl.into();
                        (h as u32) as usize
                    };
                    hs[off] = value.try_into()?;
                }
            }
        }

        match &hs[0] {
            Highlight::Normal => Ok(()),
            _ => err_at!(Invalid, st: format!("normal style is mandatory")),
        }?;
        for h in hs.iter_mut() {
            *h = match h {
                Highlight::__Fin => Highlight::Normal,
                h => h.clone()
            };
        }
        Ok(ColorScheme{ name, hs })
    }
}

#[derive(Clone, Copy, Default)]
struct Style {
    fg: Option<Color>,
    bg: Option<Color>,
    attr: Option<Attribute>,
}

impl TryFrom<toml::Value> for Style {
    type Error = Error;

    fn try_from(value: toml::Value) -> Result<Self> {
        use crate::Error::Invalid;

        let table = {
            let err = Err(Invalid(format!("bad style")));
            value.as_table().ok_or(err)?
        };
        let style: Style = Default::default();
        for (key, value) in table.iter() {
            let value = {
                let err = Err(IOError(format!("bad style {:?}", key)));
                value.as_str().ok_or(err)?
            };
            match key {
                "on" | "bg" => style.bg = Some(Style::to_color(value)?),
                "with" | "fg" => style.fg = Some(Style::to_color(value)?),
                "attr" | "attribute" => {
                    //
                    style.attr = Some(Style::to_attribute(value)?)
                }
                _ => (),
            }
        }
    }
}

impl Style {
    fn to_color(color: &str) -> Color {
        use std::iter::repeat;

        let n = color.len();
        match color {
            "reset" => Color::Reset,
            "black" => Color::Black,
            "darkgrey" | "dark-grey" | "dark_grey" => Color::DarkGrey,
            "red" => Color::Red,
            "darkred" | "dark-red" | "dark_red" => Color::DarkRed,
            "green",      Green,
            "darkgreen" | "dark-green" | "dark_green" => Color::DarkGreen,
            "yellow" => Color::Yellow,
            "darkyellow" | "dark-yellow" | "dark_yellow" => Color::DarkYellow,
            "blue" => Color::Blue,
            "darkblue" | "dark-blue" | "dark_blue" => Color::DarkBlue,
            "magenta" => Color::Magenta,
            "darkmagenta" | "dark-magenta" | "dark_magenta" => Color::DarkMagenta,
            "cyan" => Color::Cyan,
            "darkcyan" | "dark-cyan" | "dark-cyan" => Color::DarkCyan,
            "white" => Color::White,
            "grey" => Color::Grey,
            color if n == 0 => Color::Rgb { r: 0, g: 0, b: 0 },
            color => match color.chars().next() {
                Some('#') if n == 1 => Color::Rgb { r: 0, g: 0, b: 0 },
                Some('#') => {
                    let p = {
                        let iter = repeat('0').take(6.saturating_sub(n));
                        String::from_iter(iter)
                    };
                    let s = (p + color[1..]);
                    let r = u16::from_radix(&s[0..2], 16);
                    let g = u16::from_radix(&s[2..4], 16);
                    let b = u16::from_radix(&s[4..6], 16);
                    Color::Rgb { r, g, b }
                }
                Some(_) => {
                    let n: u8 = err_at!(FailConvert, color.parse())?;
                    Color::AnsiValue(n)
                }
                None => Color::Rgb { r: 0, g: 0, b: 0 },
            }
        }
    }

    fn to_attribute(attr: &str) -> Attribute {
        let attrs: Vec<&str> = if attr.contains(",") {
            attr.split(",").collect()
        } else attr.contains("|") {
            attr.split("|").collect()
        } else {
            vec![attr]
        };

        let mut attr = Attribute::Reset;
        for attr in attrs.into_iter() {
            attr |= match attr {
                "bold" => Attribute::Bold,
                "italic" => Attribute::Italic,
                "underlined" | "underline" => Attribute::Underlined,
                "reverse" => Attribute::Reverse,
            }
        }
        attr
    }
}

#[derive(Clone, Copy)]
enum Highlight {
    Normal = 0,
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
        Highlight::Normal
    }
}

impl From<String> for Highlight {
    fn from(s: String) -> Highlight {
        match s {
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
            "normal" => Highlight::Normal,
        }
    }
}

impl fmt::Display for Highlight {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self {
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
            Highlight::Normal => write!(f, "normal"),
        }
    }
}
