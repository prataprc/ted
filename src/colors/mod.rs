#[allow(unused_imports)]
use log::trace;
use toml;

use std::{convert::TryFrom, fmt, result};

use crate::{term::Style, Error, Result};

/// Default color-scheme
pub const DEFAULT: &'static str = include_str!("./default.toml");

pub fn pkg_color_schemes() -> Vec<ColorScheme> {
    vec![{
        let value: toml::Value = DEFAULT.parse().unwrap();
        TryFrom::try_from(value).unwrap()
    }]
}

/// ColorScheme for ted applications.
#[derive(Clone)]
pub struct ColorScheme {
    pub name: String,
    pub hs: Vec<Style>,
}

impl fmt::Display for ColorScheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "ColorScheme<{}>", self.name)
    }
}

impl ColorScheme {
    pub fn default() -> Result<ColorScheme> {
        let toml_style: toml::Value = err_at!(FailConvert, DEFAULT.parse())?;
        TryFrom::try_from(toml_style)
    }
}

impl TryFrom<toml::Value> for ColorScheme {
    type Error = Error;

    fn try_from(value: toml::Value) -> Result<Self> {
        let table = {
            let err = Error::Invalid(String::new(), format!("bad color sheme"));
            value.as_table().ok_or(err)?
        };

        let mut name = Default::default();
        let mut hs: Vec<Style> = {
            let mut hs = Vec::with_capacity(Highlight::__Fin as usize);
            hs.resize(hs.capacity(), Default::default());
            hs
        };

        let canvas: Style = Default::default();
        for (key, value) in table.iter() {
            match key.as_str() {
                "name" => {
                    let err = {
                        let s = format!("bad value for {}", key);
                        Error::Invalid(String::new(), s)
                    };
                    name = value.as_str().ok_or(err)?.to_string();
                }
                hl => {
                    let off = {
                        let h: Highlight = TryFrom::try_from(hl)?;
                        (h as u32) as usize
                    };
                    hs[off] = Style::from_toml(&value, &canvas)?;
                }
            }
        }

        Ok(ColorScheme { name, hs })
    }
}

impl ColorScheme {
    pub fn load_color_schemes() -> Vec<ColorScheme> {
        let to_scheme = |s: &str| -> Option<ColorScheme> {
            let toml_style: toml::Value = s.parse().ok()?;
            TryFrom::try_from(toml_style).ok()
        };

        let colors = vec![DEFAULT];
        colors.into_iter().filter_map(to_scheme).collect()
    }
}

impl ColorScheme {
    pub fn to_style(&self, hl: Highlight) -> Style {
        self.hs[hl as u32 as usize].clone()
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
    (EscapeSeq, "escape-seq"),
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
