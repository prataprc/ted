use lazy_static::lazy_static;
#[allow(unused_imports)]
use log::{debug, trace};
use tree_sitter as ts;

use std::path;

use crate::{
    buffer::Buffer, colors::ColorScheme, event::Event, location::Location, term::Spanline, Error,
    Result,
};

mod ftype_toml;
mod ftype_tss;
mod ftype_txt_plain;

pub use ftype_toml::Toml;
pub use ftype_tss::Tss;
pub use ftype_txt_plain::PlainText;

macro_rules! ftype {
    ($(($variant:ident, $t:ty, $name:expr)),*) => {
        lazy_static! {
            static ref FILE_TYPES: Vec<String> = vec![
                $($name,)*
            ];
        }

        pub enum Page {
            $($variant($t),)*
        }

        impl Page {
            pub fn to_name(&self) -> String {
                match self {
                    $(Page::$variant(_) => <$t>::to_name(),)*
                }
            }

            pub fn to_language(&self) -> Option<ts::Language> {
                match self {
                    $(Page::$variant(val) => val.to_language(),)*
                }
            }

            pub fn on_event(
                //
                &mut self, buf: &mut Buffer, evnt: Event) -> Result<Event>  {
                match self {
                    $(Page::$variant(val) => val.on_event(buf, evnt),)*
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
                    $(Page::$variant(val) => val.to_span_line(buf, scheme, from, to),)*
                }
            }
        }
    };
}

ftype![
    (PlainText, PlainText, PlainText::to_name()),
    (Toml, Toml, Toml::to_name()),
    (Tss, Tss, Tss::to_name())
];

pub fn detect_page(buf: &Buffer, scheme: &ColorScheme) -> Result<Page> {
    let loc = buf.to_location();

    let ft = match loc {
        Location::Disk(fpath) => {
            let ext = path::Path::new(&fpath).extension();
            match ext.map(|ext| ext.to_str().unwrap_or("")) {
                Some("toml") => Some("toml"),
                Some("tss") => Some("tss"),
                Some(_) | None => None,
            }
        }
        Location::Memory(_) => None,
        Location::Ted(_) => None,
    };

    // TODO: find other ways to detect the file's type.

    match ft.unwrap_or("") {
        "toml" => Ok(Page::Toml(Toml::new(buf, scheme)?)),
        "tss" => Ok(Page::Tss(Tss::new(buf, scheme)?)),
        _ => Ok(Page::PlainText(PlainText::new(buf, scheme)?)),
    }
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
