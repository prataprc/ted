use lazy_static::lazy_static;
#[allow(unused_imports)]
use log::{debug, trace};
use tree_sitter as ts;

use std::{
    convert::{TryFrom, TryInto},
    io, path,
};

use crate::{
    buffer::Buffer, colors::ColorScheme, event::Event, location::Location, term::Spanline, Error,
    Result,
};

mod toml;
mod tss;
mod txt_plain;

pub use crate::text::toml::Toml;
pub use crate::text::tss::Tss;
pub use crate::text::txt_plain::PlainText;

macro_rules! text_types {
    ($(($variant:ident, $t:ident, $name:expr)),*) => {
        lazy_static! {
            static ref FILE_TYPES: Vec<String> = vec![
                $($name.to_string(),)*
            ];
        }

        pub enum Types {
            $($variant($t),)*
        }

        impl TryFrom<(String, &Buffer, &ColorScheme)> for Types {
            type Error = Error;

            fn try_from((tt, buf, scheme): (String, &Buffer, &ColorScheme)) -> Result<Self> {
                let val = match tt.as_str() {
                    $($name => Types::$variant($t::new(buf, scheme)?),)*
                    _ => Types::PlainText(PlainText::new(buf, scheme)?),
                };
                Ok(val)
            }
        }

        impl Types {
            pub fn to_name(&self) -> &'static str {
                match self {
                    $(Types::$variant(_) => $name,)*
                }
            }

            pub fn to_language(&self) -> Option<ts::Language> {
                match self {
                    $(Types::$variant(val) => val.to_language(),)*
                }
            }

            pub fn on_event(
                //
                &mut self, buf: &mut Buffer, evnt: Event) -> Result<Event>  {
                match self {
                    $(Types::$variant(val) => val.on_event(buf, evnt),)*
                }
            }

            pub fn to_span_line(
                &self,
                buf: &Buffer,
                scheme: &ColorScheme,
                from: usize,
                to: usize,
            ) -> Result<Spanline> {
                match self {
                    $(Types::$variant(val) => {
                        //
                        val.to_span_line(buf, scheme, from, to)
                    },)*
                }
            }
        }
    };
}

text_types![
    (PlainText, PlainText, "txt-plain"),
    (Toml, Toml, "toml"),
    (Tss, Tss, "tss")
];

trait Type {
    fn to_language(&self) -> Option<ts::Language>;

    fn on_event(&mut self, buf: &mut Buffer, evnt: Event) -> Result<Event>;

    fn to_span_line(
        &self,
        buf: &Buffer,
        scheme: &ColorScheme,
        from: usize,
        to: usize,
    ) -> Result<Spanline>;
}

pub fn detect(buf: &Buffer, scheme: &ColorScheme) -> Result<Types> {
    let loc = buf.to_location();

    let tt = match &loc {
        Location::Disk { path_file, .. } => {
            let ext = path::Path::new(path_file).extension();
            match ext.map(|ext| ext.to_str().unwrap_or("")) {
                Some("toml") => "toml".to_string(),
                Some("tss") => "tss".to_string(),
                Some(_) | None => "".to_string(),
            }
        }
        Location::Memory(_) => "".to_string(),
        Location::Ted(_) => "".to_string(),
        Location::Err(_) => "".to_string(),
    };

    // TODO: find other ways to detect the file's type.

    (tt, buf, scheme).try_into()
}

pub fn new_parser(content: &str, lang: ts::Language) -> Result<(ts::Parser, ts::Tree)> {
    let mut parser = {
        let mut parser = ts::Parser::new();
        err_at!(FailParse, parser.set_language(lang))?;
        parser
    };
    let tree = parser.parse(content, None).unwrap();

    debug!("lang:{:?}\n{}", lang, tree.root_node().to_sexp());

    Ok((parser, tree))
}

pub enum Encoded {
    Utf8(String),
}

impl From<String> for Encoded {
    fn from(s: String) -> Encoded {
        Encoded::Utf8(s)
    }
}

impl From<Encoded> for String {
    fn from(enc: Encoded) -> String {
        match enc {
            Encoded::Utf8(s) => s,
        }
    }
}

impl Encoded {
    pub fn from_reader<R>(mut r: R, fenc: &str) -> Result<Encoded>
    where
        R: io::Read,
    {
        use std::str::from_utf8;

        let mut buf = vec![];
        err_at!(IOError, r.read_to_end(&mut buf))?;

        match fenc {
            "utf8" | "utf-8" => {
                let buf = err_at!(FailConvert, from_utf8(&buf))?;
                Ok(Encoded::Utf8(buf.to_string()))
            }
            _ => err_at!(Err(Error::Invalid(format!("encoding {}", fenc)))),
        }
    }

    pub fn save<W>(&self, mut w: W, _: &str) -> Result<()>
    where
        W: io::Write,
    {
        match self {
            Encoded::Utf8(s) => err_at!(IOError, w.write_all(s.as_bytes())),
        }
    }
}

pub enum Format {
    Dos,
    Mac,
    Unix,
}

impl Default for Format {
    fn default() -> Format {
        Format::Unix
    }
}

impl Format {
    pub fn newline(&self) -> &'static str {
        match self {
            Format::Dos => "\r\n",
            Format::Mac => "\r",
            Format::Unix => "\n",
        }
    }

    pub fn trim_newline(text: &str) -> (&str, usize) {
        use std::{slice::from_raw_parts, str::from_utf8_unchecked};

        let mut chars = text.chars().rev();
        let n = match (chars.next(), chars.next()) {
            (Some('\n'), Some('\r')) => 2,
            (Some('\n'), None) | (Some('\n'), Some(_)) => 1,
            (Some('\r'), None) | (Some('\r'), Some(_)) => 1,
            (_, _) => 0,
        };
        let len = text.len().saturating_sub(n);
        (
            unsafe { from_utf8_unchecked(from_raw_parts(text.as_ptr(), len)) },
            n,
        )
    }
}
