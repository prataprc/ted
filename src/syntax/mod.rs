use lazy_static::lazy_static;
#[allow(unused_imports)]
use log::{debug, error, trace, warn};
use tree_sitter as ts;

use std::{
    cmp,
    convert::{TryFrom, TryInto},
    fmt,
    iter::FromIterator,
    path, result,
};

use crate::{
    buffer::Buffer,
    colors::{ColorScheme, Highlight},
    event::Event,
    location::Location,
    term,
    tss::{Automata, Token},
    window::WinBuffer,
    Error, Result,
};

mod code_cmd;
mod toml;
mod tss;
mod txt_plain;

pub use crate::syntax::code_cmd::CodeCmd;
pub use crate::syntax::toml::Toml;
pub use crate::syntax::tss::Tss;
pub use crate::syntax::txt_plain::PlainText;

pub trait Syntax {
    fn to_language(&self) -> Option<ts::Language>;

    fn on_edit(&mut self, buf: &Buffer, evnt: Event) -> Result<Event>;

    fn to_span_line(&self, buf: &Buffer, a: usize, z: usize) -> Result<term::Spanline>;

    fn to_status_cursor(&self) -> Result<term::Span>;
}

macro_rules! syntax_for {
    ($(($variant:ident, $t:ident, $name:expr)),*) => {
        lazy_static! {
            static ref FILE_TYPES: Vec<String> = vec![
                $($name.to_string(),)*
            ];
        }

        #[derive(Clone)]
        pub enum Syn {
            $($variant($t),)*
            None,
        }

        impl<'a, 'b> TryFrom<(&'a str, &'b str, ColorScheme)> for Syn {
            type Error = Error;

            fn try_from(args: (&'a str, &'b str, ColorScheme)) -> Result<Self> {
                let (typ, s, scheme) = args;
                let val = match typ {
                    $($name => Syn::$variant($t::new(s, scheme)?),)*
                    _ => Syn::PlainText(PlainText::new(s, scheme)?),
                };
                Ok(val)
            }
        }

        impl Syn {
            pub fn as_name(&self) -> &'static str {
                match self {
                    $(Syn::$variant(_) => $name,)*
                    Syn::None => "invalid-syntax-type"
                }
            }
        }

        impl Default for Syn {
            fn default() -> Syn {
                Syn::None
            }
        }

        impl Syntax for Syn {
            fn to_language(&self) -> Option<ts::Language> {
                match self {
                    $(Syn::$variant(val) => val.to_language(),)*
                    Syn::None => None,
                }
            }

            fn on_edit(&mut self, buf: &Buffer, evnt: Event) -> Result<Event> {
                match self {
                    $(Syn::$variant(val) => val.on_edit(buf, evnt),)*
                    Syn::None => Ok(evnt)
                }
            }

            fn to_span_line(&self, buf: &Buffer, a: usize, z: usize) -> Result<term::Spanline> {
                match self {
                    $(Syn::$variant(val) => val.to_span_line(buf, a, z),)*
                    Syn::None => Ok("".to_string().into())
                }
            }

            fn to_status_cursor(&self) -> Result<term::Span> {
                match self {
                    $(Syn::$variant(val) => val.to_status_cursor(),)*
                    Syn::None => Ok("".to_string().into())
                }
            }
        }
    };
}

syntax_for![
    (Toml, Toml, "toml"),
    (Tss, Tss, "tss"),
    (CodeCmd, CodeCmd, "code_cmd"),
    (PlainText, PlainText, "txt-plain")
];

pub fn detect(buf: &Buffer, scheme: &ColorScheme) -> Result<Syn> {
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
        Location::Ted { .. } => "".to_string(),
        Location::Memory { .. } => "".to_string(),
    };

    // TODO: find other ways to detect the file's type.

    (tt.as_str(), buf.to_string().as_str(), scheme.clone()).try_into()
}

/// Syntax highlighting using tree-sitter and ted-style-sheet automata.
pub fn highlight(
    buf: &Buffer,
    scheme: &ColorScheme,
    tree: &ts::Tree,
    atmt: &mut Automata,
    from: usize,
    till: usize,
) -> Result<term::Spanline> {
    let canvas = scheme.to_style(Highlight::Canvas);
    let root = tree.root_node();
    let mut syns = {
        let (depth, sibling) = (0, 0);
        let tok = Token::from_node(buf, &root, depth, sibling);
        // trace!("{}", tok);
        match atmt.shift_in(&tok)? {
            Some(style) => vec![SyntSpan {
                depth: tok.depth,
                a: tok.a,
                z: tok.z,
                style,
            }],
            None => vec![],
        }
    };

    // trace!("highlight {}..{} syns:{:?}", from, till, syns);

    let depth = 1;
    syns.extend(do_highlight(
        buf, scheme, tree, atmt, root, depth, from, till,
    )?);
    syns.sort_by(|a, b| b.cmp(a)); // reverse sorting

    // trace!("sorted syns:{:?}", syns);
    let mut hl_spans = HlSpans::new(canvas, from, till);
    while let Some(syn) = syns.pop() {
        syns.extend(hl_spans.pop_after(&syn));
        hl_spans.push(syn)?;
        syns.sort_by(|a, b| b.cmp(a)); // reverse sorting
    }

    trace!("Hlspans {}", hl_spans);
    hl_spans.into_span_line(buf)
}

fn do_highlight(
    buf: &Buffer,
    scheme: &ColorScheme,
    tree: &ts::Tree,
    atmt: &mut Automata,
    node: ts::Node,
    depth: usize, // 0 is root level
    from: usize,  // character offset to highlight, inclusive
    till: usize,  // character offset to highlight, exclusive
) -> Result<Vec<SyntSpan>> {
    let mut syns = vec![];
    let mut tc = node.walk();

    let children: Vec<ts::Node> = node.children(&mut tc).collect();
    let mut toks = Vec::with_capacity(16);
    for (sibling, child) in children.iter().enumerate() {
        let tok = Token::from_node(buf, child, depth, sibling);
        let overlap = tok.is_overlap(from, till);
        // trace!("{} overlap:{}", tok, overlap);
        if overlap {
            match atmt.shift_in(&tok)? {
                Some(style) => syns.push(SyntSpan {
                    depth: tok.depth,
                    a: tok.a,
                    z: tok.z,
                    style,
                }),
                None => (),
            }
        }
        toks.push((tok, overlap))
    }

    // trace!("do-highlight {}..{} syns:{:?}", from, till, syns);

    for (sibling, child) in children.into_iter().enumerate() {
        if toks[sibling].1 {
            syns.extend({
                let depth = depth + 1;
                do_highlight(buf, scheme, tree, atmt, child, depth, from, till)?
            });
        }
    }

    Ok(syns)
}

// per line, list of matching spans, sort it and convert them into spanline.
struct HlSpans {
    from: usize,
    till: usize,
    canvas: term::Style, // canvas style
    syns: Vec<SyntSpan>,
}

impl fmt::Display for HlSpans {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let ss: Vec<String> = {
            let iter = self.syns.iter().map(|syn| syn.to_string());
            iter.collect()
        };
        write!(f, "{{{}}}", ss.join(","))
    }
}

impl HlSpans {
    fn new(canvas: term::Style, from: usize, till: usize) -> HlSpans {
        HlSpans {
            from,
            till,
            canvas,
            syns: Vec::default(),
        }
    }

    fn pop_after(&mut self, syn: &SyntSpan) -> Vec<SyntSpan> {
        let mut hlsyns = vec![];
        loop {
            match self.syns.pop() {
                Some(hlsyn) if &hlsyn > syn => hlsyns.push(hlsyn),
                Some(hlsyn) => {
                    self.syns.push(hlsyn);
                    break hlsyns;
                }
                None => break hlsyns,
            }
        }
    }

    fn push(&mut self, mut syn: SyntSpan) -> Result<()> {
        syn = syn.clip(self);
        match self.syns.len() {
            0 if self.from < syn.a => {
                // There is an un-matched span.
                self.syns.push(SyntSpan {
                    depth: 0,
                    a: self.from,
                    z: syn.a,
                    style: self.canvas.clone(),
                });
                self.syns.push(syn);
            }
            0 => self.syns.push(syn),
            _ => {
                // trace!("syn: {} syns: {:?}", syn, self.syns);
                let SyntSpan { depth, a, z, style } = self.syns.pop().unwrap();
                assert!(a <= syn.a, "{}..{} {}", a, z, syn);
                if z < syn.a {
                    // there is a gap between two syntax-span.
                    // |..old..| <gap> |..new..|
                    self.syns.push(SyntSpan { depth, a, z, style });
                    self.syns.push(SyntSpan {
                        depth: 0,
                        a: z,
                        z: syn.a,
                        style: self.canvas.clone(),
                    });
                    self.syns.push(syn);
                } else if z == syn.a {
                    // perfect alignment.
                    // |..old..|..new..|
                    self.syns.push(SyntSpan { depth, a, z, style });
                    self.syns.push(syn)
                } else if a <= syn.a && syn.z <= z && syn.depth > depth {
                    // fully contained
                    // |..old..|..new..|..old..|
                    SyntSpan {
                        depth,
                        a,
                        z: syn.a,
                        style: style.clone(),
                    }
                    .filter()
                    .map(|span| self.syns.push(span));
                    self.syns.push(syn.clone());
                    SyntSpan {
                        depth,
                        a: syn.z,
                        z,
                        style,
                    }
                    .filter()
                    .map(|span| self.syns.push(span));
                } else if a <= syn.a && syn.z <= z {
                    // fully contained
                    // |..old..|
                    self.syns.push(SyntSpan {
                        depth,
                        a,
                        z,
                        style: style.clone(),
                    });
                } else if syn.depth >= depth {
                    // overlap, incoming span is more specific
                    // |..old|..|new..|
                    self.syns.push(SyntSpan {
                        depth,
                        a,
                        z: syn.a,
                        style,
                    });
                    self.syns.push(syn);
                } else {
                    // overlap, existing span is more specific.
                    // |..old|..|new..|
                    self.syns.push(SyntSpan { depth, a, z, style });
                    syn.a = z;
                    self.syns.push(syn)
                }
            }
        }

        Ok(())
    }

    fn into_span_line(&mut self, buf: &Buffer) -> Result<term::Spanline> {
        match self.syns.pop() {
            Some(SyntSpan { depth, a, z, style }) if z < self.till => {
                self.syns.push(SyntSpan { depth, a, z, style });
                self.syns.push({
                    let style = self.canvas.clone();
                    SyntSpan {
                        depth: 0,
                        a: z,
                        z: self.till,
                        style,
                    }
                });
            }
            Some(SyntSpan { depth, a, z, style }) if z == self.till => {
                self.syns.push(SyntSpan { depth, a, z, style });
            }
            Some(SyntSpan { z, .. }) => {
                err_at!(Fatal, msg: format!("misaligned {} {}", z, self.till))?
            }
            None => {
                self.syns.push({
                    let style = self.canvas.clone();
                    SyntSpan {
                        depth: 0,
                        a: self.from,
                        z: self.till,
                        style,
                    }
                });
            }
        }

        let mut spans: Vec<term::Span> = vec![];
        for syn in self.syns.iter_mut() {
            spans.push(syn.into_span(buf)?);
        }
        Ok(spans.into_iter().collect())
    }
}

// matching syntax span with tss-automata, represents a single span/style.
#[derive(Clone)]
struct SyntSpan {
    depth: usize,
    a: usize, // character position, inclusive
    z: usize, // character position, exclusive
    style: term::Style,
}

impl fmt::Display for SyntSpan {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "[{},{}..{}]", self.depth, self.a, self.z)
    }
}

impl fmt::Debug for SyntSpan {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "[{},{}..{},{}]", self.depth, self.a, self.z, self.style)
    }
}

impl Eq for SyntSpan {}

impl PartialEq for SyntSpan {
    fn eq(&self, other: &Self) -> bool {
        self.a.eq(&other.a) && self.z.eq(&other.z)
    }
}

impl PartialOrd for SyntSpan {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match self.a.partial_cmp(&other.a) {
            Some(cmp::Ordering::Equal) => {
                let (m, n) = (self.z - self.a, other.z - other.a);
                match m.partial_cmp(&n) {
                    Some(cmp::Ordering::Equal) => {
                        //
                        self.depth.partial_cmp(&other.depth)
                    }
                    cval => cval,
                }
            }
            cval => cval,
        }
    }
}

// a. lower value of start_char_idx sort before.
// b. if start_char_idx is equal sort by length.
// c. if length is equal sort by depth, more deeper (specific) span sort after.
impl Ord for SyntSpan {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match self.a.cmp(&other.a) {
            cmp::Ordering::Equal => {
                let (m, n) = (self.z - self.a, other.z - other.a);
                match m.cmp(&n) {
                    cmp::Ordering::Equal => self.depth.cmp(&other.depth),
                    cval => cval,
                }
            }
            cval => cval,
        }
    }
}

impl SyntSpan {
    fn into_span(&mut self, buf: &Buffer) -> Result<term::Span> {
        use crate::event::DP;

        let span: term::Span = {
            let iter = buf.chars_at(self.a, DP::Right)?.take(self.z - self.a);
            String::from_iter(iter).into()
        };
        // warn!("SyntSpan.into_span {}, style:{}", self, self.style);
        Ok(span.using(self.style.clone()))
    }

    fn clip(mut self, hl_spans: &HlSpans) -> SyntSpan {
        if hl_spans.from > self.a {
            self.a = hl_spans.from
        }
        if hl_spans.till < self.z {
            self.z = hl_spans.till
        }
        self
    }

    fn filter(self) -> Option<SyntSpan> {
        if self.a < self.z {
            Some(self)
        } else {
            None
        }
    }
}
