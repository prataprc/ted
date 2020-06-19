#[allow(unused_imports)]
use log::trace;
use toml;

use std::{convert::TryFrom, fmt, result};

use crate::{term::Style, Error, Result};

/// Colorscheme for ted applications.
pub struct ColorScheme {
    name: String,
    hs: Vec<Style>,
}

impl fmt::Display for ColorScheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "ColorScheme<{}>", self.name)
    }
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

        let canvas: Style = Default::default();
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
                    hs[off] = Style::from_toml(&value, &canvas)?;
                    // trace!("convert {} {} {:?}", hl, off, hs[off]);
                }
            }
        }

        Ok(ColorScheme { name, hs })
    }
}

//impl ColorScheme {
//    fn load_color_schemes() -> Vec<ColorScheme> {
//        include_str!("../colors")
//    }
//}

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
