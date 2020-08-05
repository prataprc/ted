#[allow(unused_imports)]
use log::{debug, trace};
use tree_sitter as ts;

use std::{borrow::Borrow, convert::TryFrom, fmt, mem, rc::Rc, result};

use crate::{
    buffer::Buffer,
    colors::{ColorScheme, Highlight},
    term::Style,
    Error, Result,
};

/// Ted style sheet for `toml` format.
pub const TOML: &'static str = include_str!("toml.tss");

/// Ted style sheet for `tss` format, tss stands for ted-style-sheet.
pub const TSS: &'static str = include_str!("tss.tss");

/// Ted style sheet for `tss` format, tss stands for ted-style-sheet.
pub const CODE_CMD: &'static str = include_str!("code_cmd.tss");

macro_rules! wrap_edge {
    ($edge:expr, $varn:ident) => {{
        *$edge = match mem::replace($edge, Default::default()) {
            e @ Edge::Kind(_) => Edge::$varn(Box::new(e.clone())),
            _ => err_at!(Fatal, msg: format!("unexpected wrap_edge"))?,
        };
        Ok(())
    }};
}

extern "C" {
    fn tree_sitter_tss() -> ts::Language;
}

pub struct Token {
    pub kind: String,
    pub depth: usize,
    pub sibling: usize,
    pub a: usize, // charactor position, inclusive
    pub z: usize, // charactor position, exclusive
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(
            f,
            "Token<{},{},{},{}..{}>",
            self.kind, self.depth, self.sibling, self.a, self.z
        )
    }
}

impl Token {
    pub fn from_node(buf: &Buffer, nd: &ts::Node, d: usize, s: usize) -> Token {
        let kind = nd.kind().to_string();
        let a = buf.byte_to_char(nd.start_byte());
        let z = buf.byte_to_char(nd.end_byte());
        // trace!("{:?} {} {}", nd, nd.start_byte(), nd.end_byte());
        Token {
            kind,
            depth: d,
            sibling: s,
            a,
            z,
        }
    }

    // typically from..till is the line span.
    #[inline]
    pub fn is_overlap(&self, from: usize, till: usize) -> bool {
        !(self.a >= till || self.z <= from)
    }
}

#[derive(Clone)]
enum Span {
    Pos(usize, usize), // (inclusive-start, inclusive-end)
    Text(String),
}

impl Default for Span {
    fn default() -> Span {
        Span::Pos(0, 0)
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        use Span::{Pos, Text};

        match self {
            Pos(a, z) => write!(f, "{},{}", *a, *z),
            Text(txt) => write!(f, "{}", txt),
        }
    }
}

impl Span {
    fn from_node(n: &ts::Node) -> Span {
        Span::Pos(n.start_byte(), n.end_byte())
    }
}

impl Span {
    fn pos_to_text(self, tss: &str) -> Self {
        match self {
            Span::Pos(a, z) => Span::Text(tss[a..z].to_string()),
            val @ Span::Text(_) => val,
        }
    }

    fn as_text(&self) -> Result<&str> {
        match self {
            Span::Pos(_, _) => err_at!(Fatal, msg: format!("unexpected span")),
            Span::Text(txt) => Ok(txt),
        }
    }
}

#[derive(Clone)]
pub struct Automata {
    name: String,
    patterns: Vec<Rc<Node>>,
    patterns_trie: KindTrie,
    open_nodes: Vec<Node>,
}

impl fmt::Display for Automata {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "tss-Automata<{},{}>\n", self.name, self.patterns.len())?;
        for node in self.patterns.iter() {
            write!(f, "{}\n", node)?;
        }
        Ok(())
    }
}

impl Automata {
    pub fn from_str(name: &str, tss: &str, scheme: &ColorScheme) -> Result<Automata> {
        let tree = {
            let mut p = ts::Parser::new();
            let language = unsafe { tree_sitter_tss() };
            err_at!(FailParse, p.set_language(language))?;
            match p.parse(tss, None) {
                Some(tree) => Ok(tree),
                None => err_at!(Fatal, msg: format!("invalid ted style sheet")),
            }?
        };

        let root = {
            assert_eq!(tree.root_node().kind(), "s");
            tree.root_node()
        };

        let mut tc = root.walk();
        let mut patterns = vec![];
        let mut kinds = vec![];
        for i in 0..root.child_count() {
            let child = root.child(i).unwrap();
            if child.kind() != "hl_rule" {
                continue;
            }

            let style = {
                let ts_node = child.child(2).unwrap();
                Node::compile_style(ts_node, tss, &mut tc, scheme)?
            };
            let n_selectors: Vec<ts::Node> = {
                let xs = child.child_by_field_name("selectors").unwrap();
                xs.children(&mut tc)
                    .enumerate()
                    .filter_map(|(i, c)| if i % 2 == 0 { Some(c) } else { None })
                    .collect()
            };
            for n_sel in n_selectors.into_iter() {
                let style = style.clone();
                let node = Node::compile_pattern(n_sel, tss, style, &mut tc)?;
                match &node {
                    Node::Pattern(Edge::Kind(k), _) => {
                        let s = k.as_text()?.to_string();
                        kinds.push(s);
                    }
                    _ => (),
                };
                patterns.push(Rc::new(node))
            }
        }

        Ok(Automata {
            name: name.to_string(),
            patterns,
            patterns_trie: Self::build_trie(kinds),
            open_nodes: Vec::default(),
        })
    }

    fn build_trie(kinds: Vec<String>) -> KindTrie {
        let initial = KindTrie::default();
        kinds
            .into_iter()
            .enumerate()
            .fold(initial, |trie, item| trie.merge(item.into()))
    }
}

impl Automata {
    pub fn shift_in(&mut self, token: &Token) -> Result<Option<Style>> {
        // check whether there is a match with open-patterns.
        let mut style1: Option<Style> = None;
        let mut ops = vec![];

        // trace!("open_nodes: {:?}", self.open_nodes);
        for (off, open_node) in self.open_nodes.iter().enumerate() {
            style1 = match open_node.is_match(token)? {
                (Some(Node::End(style)), true) => {
                    ops.push((off, None));
                    Some(style1.unwrap_or(style))
                }
                (Some(Node::End(style)), _) => Some(style1.unwrap_or(style)),
                (Some(next), _) => {
                    ops.push((off, Some(next)));
                    style1
                }
                (None, true) => {
                    ops.push((off, None));
                    style1
                }
                (None, false) => style1,
            }
        }

        // trace!("ops: {:?}", ops);
        for (off, next) in ops.into_iter().rev() {
            let _ = match next {
                Some(next) => mem::replace(&mut self.open_nodes[off], next),
                None => self.open_nodes.remove(off),
            };
        }

        let msg = format!("unreachable");
        let style2 = match self.match_pattern(&token) {
            Some(Node::End(style)) => Some(style),
            Some(Node::Pattern(_, n)) => {
                let n: &Node = n.borrow();
                self.open_nodes.push(n.to_open_node(token)?);
                None
            }
            Some(Node::Twin { .. }) => err_at!(Fatal, msg: msg)?,
            Some(Node::Sibling { .. }) => err_at!(Fatal, msg: msg)?,
            Some(Node::Child { .. }) => err_at!(Fatal, msg: msg)?,
            Some(Node::Descendant { .. }) => err_at!(Fatal, msg: msg)?,
            None => None,
        };

        if let Some(style) = style1 {
            Ok(Some(style))
        } else {
            Ok(style2)
        }
    }

    fn match_pattern(&self, token: &Token) -> Option<Node> {
        let chars: Vec<char> = token.kind.chars().collect();
        let node: &Node = match self.patterns_trie.lookup(&chars) {
            Trie::None => None,
            Trie::Index(i) => Some(self.patterns[i].borrow()),
            Trie::Table(_) => None,
            Trie::IndexTable(i, _) => Some(self.patterns[i].borrow()),
        }?;
        match node {
            Node::Pattern(_, n) => {
                let n: &Node = n.borrow();
                Some(n.clone())
            }
            _ => None,
        }
    }
}

#[derive(Clone)]
enum Edge {
    Kind(Span),
    Twin(Box<Edge>),
    Sibling(Box<Edge>),
    Child(Box<Edge>),
    Descendant(Box<Edge>),
}

impl Default for Edge {
    fn default() -> Edge {
        Edge::Kind(Span::default())
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        use Edge::{Child, Descendant, Kind, Sibling, Twin};

        match self {
            Kind(span) => write!(f, "e-kind<{}>", span),
            Twin(edge) => write!(f, "e-twin<{}>", edge),
            Sibling(edge) => write!(f, "e-sibling<{}>", edge),
            Child(edge) => write!(f, "e-child<{}>", edge),
            Descendant(edge) => write!(f, "e-descendant<{}>", edge),
        }
    }
}

impl Edge {
    fn is_match(&self, token: &Token) -> Result<bool> {
        use Edge::{Child, Descendant, Kind, Sibling, Twin};

        match self {
            Kind(k) => Ok(token.kind == k.as_text()?),
            Twin(_) => err_at!(Fatal, msg: format!("unreachable")),
            Sibling(_) => err_at!(Fatal, msg: format!("unreachable")),
            Child(_) => err_at!(Fatal, msg: format!("unreachable")),
            Descendant(_) => err_at!(Fatal, msg: format!("unreachable")),
        }
    }
}

#[derive(Clone)]
enum Node {
    Pattern(Edge, Rc<Node>),
    Twin {
        edge: Edge,
        next: Rc<Node>,
        depth: usize,
        nth_child: usize,
    },
    Sibling {
        edge: Edge,
        next: Rc<Node>,
        depth: usize,
        nth_child: usize,
    },
    Child {
        edge: Edge,
        next: Rc<Node>,
        depth: usize,
        nth_child: usize,
    },
    Descendant {
        edge: Edge,
        next: Rc<Node>,
        depth: usize,
        nth_child: usize,
    },
    End(Style),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        use Node::{Child, Descendant, End, Pattern, Sibling, Twin};

        match self {
            Pattern(edge, node) => write!(f, "Pattern<{}> -> {}", edge, node),
            Twin {
                edge,
                next,
                depth,
                nth_child,
            } => write!(f, "Twin<{},{},{}> -> {}", edge, depth, nth_child, next),
            Sibling {
                edge,
                next,
                depth,
                nth_child,
            } => write!(
                f,
                "Sibling<{},{},{}> -> {}",
                edge, depth, /**/ nth_child, next
            ),
            Child {
                edge,
                next,
                depth,
                nth_child,
            } => write!(f, "Child<{},{},{}> -> {}", edge, depth, nth_child, next),
            Descendant {
                edge,
                next,
                depth,
                nth_child,
            } => write!(
                f,
                "Descendant<{},{},{}> -> {}",
                edge, depth, nth_child, next
            ),
            End(style) => write!(f, "End<{}>", style),
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl Node {
    fn to_open_node(&self, token: &Token) -> Result<Node> {
        match self {
            Node::Pattern(edge, next) => match edge {
                Edge::Kind(_) => err_at!(Fatal, msg: format!("unreachable")),
                Edge::Twin(ne) => Ok(Node::Twin {
                    edge: ne.as_ref().clone(),
                    next: Rc::clone(next),
                    nth_child: token.sibling,
                    depth: token.depth,
                }),
                Edge::Sibling(ne) => Ok(Node::Sibling {
                    edge: ne.as_ref().clone(),
                    next: Rc::clone(next),
                    nth_child: token.sibling,
                    depth: token.depth,
                }),
                Edge::Child(ne) => Ok(Node::Child {
                    edge: ne.as_ref().clone(),
                    next: Rc::clone(next),
                    nth_child: token.sibling,
                    depth: token.depth,
                }),
                Edge::Descendant(ne) => Ok(Node::Descendant {
                    edge: ne.as_ref().clone(),
                    next: Rc::clone(next),
                    nth_child: token.sibling,
                    depth: token.depth,
                }),
            },
            node @ Node::End(_) => Ok(node.clone()),
            Node::Twin { .. } => err_at!(Fatal, msg: format!("unreachable")),
            Node::Sibling { .. } => err_at!(Fatal, msg: format!("unreachable")),
            Node::Child { .. } => err_at!(Fatal, msg: format!("unreachable")),
            Node::Descendant { .. } => err_at!(Fatal, msg: format!("unreachbl")),
        }
    }

    fn is_match(&self, token: &Token) -> Result<(Option<Node>, bool)> {
        let (ok, drop, next) = match self {
            Node::Pattern(_, _) => return Ok((None, false)),
            Node::Twin {
                edge,
                next,
                depth,
                nth_child,
            } => {
                let ok1 = token.depth == *depth;
                let ok2 = token.sibling == nth_child + 1;
                let ok3 = edge.is_match(token)?;
                (ok1 && ok2 && ok3, !(ok1 && ok2), next)
            }
            Node::Sibling {
                edge,
                next,
                depth,
                nth_child,
            } => {
                let ok1 = token.depth == *depth;
                let ok2 = token.sibling > *nth_child;
                let ok3 = edge.is_match(token)?;
                (ok1 && ok2 && ok3, !ok1, next)
            }
            Node::Child {
                edge, next, depth, ..
            } => {
                let ok1 = token.depth == *depth + 1;
                let ok3 = edge.is_match(token)?;
                (ok1 && ok3, token.depth > (*depth + 1), next)
            }
            Node::Descendant {
                edge, next, depth, ..
            } => {
                let ok1 = *depth < token.depth;
                let ok3 = edge.is_match(token)?;
                (ok1 && ok3, false, next)
            }
            Node::End(_) => return Ok((None, false)),
        };

        // trace!("node.is_match {} {}", ok, drop);
        if ok {
            Ok((Some(next.to_open_node(token)?), drop))
        } else {
            Ok((None, drop))
        }
    }

    fn as_mut_edge(&mut self) -> &mut Edge {
        use Node::{Child, Descendant, End, Pattern, Sibling, Twin};

        match self {
            Pattern(edge, _) => edge,
            Twin { edge, .. } => edge,
            Sibling { edge, .. } => edge,
            Child { edge, .. } => edge,
            Descendant { edge, .. } => edge,
            End(_) => unreachable!(),
        }
    }
}

impl Node {
    fn compile_style<'a>(
        ts_node: ts::Node<'a>,
        tss: &str,
        tc: &mut ts::TreeCursor<'a>,
        scheme: &ColorScheme,
    ) -> Result<Node> {
        let canvas = scheme.to_style(Highlight::Canvas);
        let style = match ts_node.kind() {
            "highlight" => {
                let cont = {
                    let nd = ts_node.child(0).unwrap();
                    Span::from_node(&nd).pos_to_text(tss)
                };
                match cont {
                    Span::Text(hl) => {
                        let hl: Highlight = TryFrom::try_from(hl.as_str())?;
                        Ok(scheme.to_style(hl))
                    }
                    _ => err_at!(Fatal, msg: format!("unexpected style")),
                }?
            }
            "properties" => {
                let mut style: Style = scheme.to_style(Highlight::Canvas);
                let sp_nodes: Vec<ts::Node> = ts_node
                    .children(tc)
                    .enumerate()
                    .filter_map(|(i, c)| {
                        if i % 2 == 1 {
                            Some(c.child(0).unwrap())
                        } else {
                            None
                        }
                    })
                    .collect();
                for nprop in sp_nodes.into_iter() {
                    let cont = {
                        let nd = nprop.child(2).unwrap();
                        Span::from_node(&nd).pos_to_text(tss)
                    };
                    match nprop.kind() {
                        "fg" => {
                            style.fg = match &cont {
                                Span::Text(color) => {
                                    let fg = Style::to_color(color, &canvas)?;
                                    Ok(fg)
                                }
                                _ => err_at!(Fatal, msg: format!("unexpected")),
                            }?;
                        }
                        "bg" => {
                            style.bg = match &cont {
                                Span::Text(color) => {
                                    let bg = Style::to_color(color, &canvas)?;
                                    Ok(bg)
                                }
                                _ => err_at!(Fatal, msg: format!("unexpected")),
                            }?;
                        }
                        "attrb" | "attribute" => {
                            style.attrs = match &cont {
                                Span::Text(attrs) => Ok(Style::to_attrs(attrs)?),
                                _ => err_at!(Fatal, msg: format!("unexpected")),
                            }?;
                        }
                        _ => err_at!(Fatal, msg: format!("unexpected"))?,
                    }
                }
                style
            }
            kind => err_at!(Fatal, msg: format!("unexpected {:?}", kind))?,
        };

        Ok(Node::End(style))
    }

    fn compile_pattern<'a>(
        ts_node: ts::Node<'a>,
        tss: &str,
        mut next: Node,
        tc: &mut ts::TreeCursor<'a>,
    ) -> Result<Node> {
        match ts_node.child_count() {
            0 => err_at!(Fatal, msg: format!("unexpected node")),
            1 => Self::compile_sel(ts_node.child(0).unwrap(), tss, next, tc),
            _ => {
                let mut cs: Vec<ts::Node> = ts_node.children(tc).collect();
                cs.reverse();
                let mut iter = cs.into_iter();
                next = Self::compile_sel(iter.next().unwrap(), tss, next, tc)?;
                for child in iter {
                    wrap_edge!(next.as_mut_edge(), Descendant)?;
                    next = Self::compile_sel(child, tss, next, tc)?;
                }
                Ok(next)
            }
        }
    }

    fn compile_sel<'a>(
        ts_node: ts::Node<'a>,
        tss: &str,
        mut next: Node,
        tc: &mut ts::TreeCursor<'a>,
    ) -> Result<Node> {
        let cs: Vec<ts::Node> = ts_node.children(tc).collect();

        let chd = &cs[0];
        match chd.kind() {
            "sel_kind" => {
                let edge = Edge::Kind(Span::from_node(&chd).pos_to_text(tss));
                Ok(Node::Pattern(edge, Rc::new(next)))
            }
            "sel_twins" => {
                next = Self::compile_sel(chd.child(2).unwrap(), tss, next, tc)?;
                wrap_edge!(next.as_mut_edge(), Twin)?;
                Self::compile_sel(chd.child(0).unwrap(), tss, next, tc)
            }
            "sel_siblings" => {
                next = Self::compile_sel(chd.child(2).unwrap(), tss, next, tc)?;
                wrap_edge!(next.as_mut_edge(), Sibling)?;
                Self::compile_sel(chd.child(0).unwrap(), tss, next, tc)
            }
            "sel_child" => {
                next = Self::compile_sel(chd.child(2).unwrap(), tss, next, tc)?;
                wrap_edge!(next.as_mut_edge(), Child)?;
                Self::compile_sel(chd.child(0).unwrap(), tss, next, tc)
            }
            kind => err_at!(Fatal, msg: format!("unexpected {}", kind)),
        }
    }
}

#[derive(Clone)]
struct KindTrie(Vec<Trie>);

impl Default for KindTrie {
    fn default() -> Self {
        KindTrie(vec![Trie::default(); 128])
    }
}

impl From<(usize, String)> for KindTrie {
    fn from((index, selector): (usize, String)) -> Self {
        let mut chars = selector.chars().rev();
        let mut tries = match chars.next() {
            Some(ch) => (ch, index).into(),
            None => KindTrie::default(),
        };
        for ch in chars {
            tries = (ch, tries).into();
        }
        tries
    }
}

impl From<(char, usize)> for KindTrie {
    fn from((ch, index): (char, usize)) -> Self {
        let mut tries = KindTrie::default();
        let off = ch as usize;
        tries.0[off] = index.into();
        tries
    }
}

impl From<(char, Self)> for KindTrie {
    fn from((ch, child_ka): (char, Self)) -> Self {
        let mut tries = KindTrie::default();
        let off = ch as usize;
        tries.0[off] = child_ka.into();
        tries
    }
}

impl KindTrie {
    fn merge(self, other: Self) -> Self {
        use Trie::{Index, IndexTable, Table};

        let iter = self.0.into_iter().zip(other.0.into_iter());
        let tries: Vec<Trie> = iter
            .map(|item| match item {
                (Trie::None, trie) => trie,
                (trie, Trie::None) => trie,
                (Index(_), trie @ Index(_)) => trie,
                (Index(idx), Table(tries)) => IndexTable(idx, tries),
                (Index(_), trie @ IndexTable(_, _)) => trie,
                (Table(tries), Index(idx)) => IndexTable(idx, tries),
                (Table(x), Table(y)) => Table(Box::new(x.merge(*y))),
                (Table(x), IndexTable(idx, y)) => {
                    let trie = Box::new(x.merge(*y));
                    IndexTable(idx, trie)
                }
                (IndexTable(_, tries), Index(idx)) => IndexTable(idx, tries),
                (IndexTable(idx, x), Table(y)) => {
                    let trie = Box::new(x.merge(*y));
                    IndexTable(idx, trie)
                }
                (IndexTable(_, x), IndexTable(idx, y)) => {
                    let trie = Box::new(x.merge(*y));
                    IndexTable(idx, trie)
                }
            })
            .collect();
        KindTrie(tries)
    }

    fn lookup(&self, chars: &[char]) -> Trie {
        use Trie::{Index, IndexTable, Table};

        let n = chars.len();
        match chars.first() {
            Some(ch) => match &self.0[(*ch as usize)] {
                Index(index) if n == 1 => Trie::Index(*index),
                IndexTable(index, _) if n == 1 => Trie::Index(*index),
                Table(tries) if n > 1 => tries.lookup(&chars[1..]),
                IndexTable(_, tries) if n > 1 => tries.lookup(&chars[1..]),
                _ => Trie::default(),
            },
            None => Trie::default(),
        }
    }
}

#[derive(Clone)]
enum Trie {
    Index(usize),
    Table(Box<KindTrie>),
    IndexTable(usize, Box<KindTrie>),
    None,
}

impl Default for Trie {
    fn default() -> Self {
        Trie::None
    }
}

impl From<usize> for Trie {
    fn from(index: usize) -> Self {
        Trie::Index(index)
    }
}

impl From<KindTrie> for Trie {
    fn from(tries: KindTrie) -> Self {
        Trie::Table(Box::new(tries))
    }
}

impl From<(usize, KindTrie)> for Trie {
    fn from((index, tries): (usize, KindTrie)) -> Self {
        Trie::IndexTable(index, Box::new(tries))
    }
}

#[cfg(test)]
#[path = "tss_test.rs"]
mod tss_test;
