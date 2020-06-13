use tree_sitter as ts;

use std::{convert::TryFrom, fmt, mem, rc::Rc, result};

use crate::{
    color_scheme::{ColorScheme, Highlight, Style},
    Error, Result,
};

extern "C" {
    fn tree_sitter_tss() -> ts::Language;
}

macro_rules! wrap_edge {
    ($edge:expr, $varn:ident) => {{
        *$edge = match mem::replace($edge, Default::default()) {
            e @ Edge::Kind(_) => Edge::$varn(Box::new(e.clone())),
            e @ Edge::Field(_) => Edge::$varn(Box::new(e.clone())),
            e @ Edge::KindField(_, _) => Edge::$varn(Box::new(e.clone())),
            _ => err_at!(Fatal, msg: format!("unexpected wrap_edge"))?,
        };
        Ok(())
    }};
}

pub struct Token {
    kind: String,
    field: Option<String>,
    sibling: usize,
    depth: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(
            f,
            "Token<{},{},{}.{}>",
            self.depth,
            self.sibling,
            self.kind,
            self.field.as_ref().map(String::as_str).unwrap_or("")
        )
    }
}

//impl Token {
//    fn match(&self, edge: &Edge) -> bool {
//        match edge {
//            Kind(k) if self.kind == k.as_text() => true,
//            Field(f) => self.field.map(|field| field == f).unwrap_or(false),
//            Field(f) if self.field.unwrap() == f.as_text() => true,
//            KindField(k, f) {
//                let ok1 = self.kind == k.as_text();
//                let ok2 = self.field.map(|field| field == f).unwrap_or(false);
//                ok1 && ok2
//            }
//            _ => false,
//        }
//    }
//}

#[derive(Clone)]
enum Span {
    Pos(usize, usize),
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
            Pos(a, z) => write!(f, "tssSpan<{},{}>", *a, *z),
            Text(txt) => write!(f, "tssSpan<{}>", txt),
        }
    }
}

impl Span {
    fn from_node(n: &ts::Node) -> Span {
        Span::Pos(n.start_byte(), n.end_byte())
    }
}

impl Span {
    fn pos_to_text(&mut self, text: &str) -> Result<()> {
        match self {
            Span::Pos(a, z) => {
                *self = Span::Text(text[*a..*z].to_string());
                Ok(())
            }
            Span::Text(_) => err_at!(Fatal, msg: format!("unexpected span")),
        }
    }

    fn to_position(&self) -> Result<(usize, usize)> {
        match self {
            Span::Pos(a, z) => Ok((*a, *z)),
            Span::Text(_) => err_at!(Fatal, msg: format!("unexpected span")),
        }
    }

    fn as_text(&self) -> Result<&str> {
        match self {
            Span::Pos(_, _) => err_at!(Fatal, msg: format!("unexpected span")),
            Span::Text(txt) => Ok(txt),
        }
    }
}

pub struct Automata {
    patterns: Vec<Rc<Node>>,
    edges: Vec<Node>,
}

impl Automata {
    pub fn from_str(text: &str, scheme: &ColorScheme) -> Result<Automata> {
        let tree = {
            let mut p = ts::Parser::new();
            let language = unsafe { tree_sitter_tss() };
            err_at!(FailParse, p.set_language(language))?;
            match p.parse(text, None) {
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
        for i in 0..root.child_count() {
            let child = root.child(i).unwrap();
            if child.kind() != "hl_rule" {
                continue;
            }

            let style = {
                let ts_node = child.child_by_field_name("style").unwrap();
                Node::compile_style(ts_node, text, &mut tc, scheme)?
            };
            let n_selectors: Vec<ts::Node> = {
                let xs = child.child_by_field_name("selectors").unwrap();
                xs.children(&mut tc).collect()
            };
            for n_sel in n_selectors.into_iter() {
                let style = style.clone();
                patterns.push(Rc::new(Node::compile_pattern(
                    n_sel,
                    style.clone(),
                    &mut tc,
                )?))
            }
        }

        Ok(Automata {
            patterns,
            edges: Default::default(),
        })
    }
}

impl fmt::Display for Automata {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        for node in self.patterns.iter() {
            write!(f, "{}\n", node);
        }
        Ok(())
    }
}

//impl Automata {
//    fn apply(&mut self, token, Token) -> Option<Span> {
//        use Node::{Select, Twin, Sibling, Child, Descendant, Pattern, End };
//
//        for node in self.edges.iter() {
//            match node {
//                Select { edge, next, .. } if token.match(edge) => {
//                    next.to_()
//                }
//                Twin { .. } | Sibling { .. } =>  unreachable!(),
//                Child { .. } | Descendant { .. } =>  unreachable!(),
//                Pattern { .. } | End { .. } => unreachable!(),
//            }
//        }
//    }
//}

#[derive(Clone)]
enum Edge {
    Kind(Span),
    Field(Span),
    KindField(Span, Span),
    Twin(Box<Edge>),
    Sibling(Box<Edge>),
    Child(Box<Edge>),
    Descendant(Box<Edge>),
}

impl Default for Edge {
    fn default() -> Edge {
        Edge::Kind(Default::default())
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        use Edge::{Child, Descendant, Field, Kind, KindField, Sibling, Twin};

        match self {
            Kind(_) => write!(f, "e-kind"),
            Field(_) => write!(f, "e-field"),
            KindField(_, _) => write!(f, "e-kindf"),
            Twin(edge) => write!(f, "e-twin<{}>", edge),
            Sibling(edge) => write!(f, "e-sibling<{}>", edge),
            Child(edge) => write!(f, "e-child<{}>", edge),
            Descendant(edge) => write!(f, "e-descendant<{}>", edge),
        }
    }
}

impl Edge {
    fn pos_to_text(&mut self, text: &str) -> Result<()> {
        use Edge::{Child, Descendant, Field, Kind, KindField, Sibling, Twin};

        match self {
            Kind(cnt) => cnt.pos_to_text(text)?,
            Field(cnt) => cnt.pos_to_text(text)?,
            KindField(x, y) => {
                x.pos_to_text(text)?;
                y.pos_to_text(text)?;
            }
            Twin(edge) => edge.as_mut().pos_to_text(text)?,
            Sibling(edge) => edge.as_mut().pos_to_text(text)?,
            Child(edge) => edge.as_mut().pos_to_text(text)?,
            Descendant(edge) => edge.as_mut().pos_to_text(text)?,
        }
        Ok(())
    }
}

#[derive(Clone)]
enum Node {
    Pattern(Edge, Rc<Node>),
    Select {
        edge: Edge,
        next: Rc<Node>,
    },
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
        use Node::{Child, Descendant, End, Pattern, Select, Sibling, Twin};

        match self {
            Pattern(edge, node) => write!(f, "Pattern<{}> -> {}", edge, node),
            Select { edge, next } => write!(f, "Select<{}> -> {}", edge, next),
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

impl Node {
    fn to_select(&self) -> Result<Node> {
        use Node::{Pattern, Select};

        match self {
            Pattern(edge, next) => Ok(Select {
                edge: edge.clone(),
                next: Rc::clone(next),
            }),
            _ => err_at!(Fatal, msg: format!("invalid node"))?,
        }
    }

    fn to_twin(&self, nth_child: usize, depth: usize) -> Result<Node> {
        use Node::{Pattern, Twin};

        match self {
            Pattern(edge, next) => Ok(Twin {
                edge: edge.clone(),
                next: Rc::clone(next),
                nth_child,
                depth,
            }),
            _ => err_at!(Fatal, msg: format!("invalid node"))?,
        }
    }

    fn to_sibling(&self, nth_child: usize, depth: usize) -> Result<Node> {
        use Node::{Pattern, Sibling};

        match self {
            Pattern(edge, next) => Ok(Sibling {
                edge: edge.clone(),
                next: Rc::clone(next),
                nth_child,
                depth,
            }),
            _ => err_at!(Fatal, msg: format!("invalid node"))?,
        }
    }

    fn to_child(&self, nth_child: usize, depth: usize) -> Result<Node> {
        use Node::{Child, Pattern};

        match self {
            Pattern(edge, next) => Ok(Child {
                edge: edge.clone(),
                next: Rc::clone(next),
                nth_child,
                depth,
            }),
            _ => err_at!(Fatal, msg: format!("invalid node"))?,
        }
    }

    fn to_descendant(&self, nth_child: usize, depth: usize) -> Result<Node> {
        use Node::{Descendant, Pattern};

        match self {
            Pattern(edge, next) => Ok(Descendant {
                edge: edge.clone(),
                next: Rc::clone(next),
                nth_child,
                depth,
            }),
            _ => err_at!(Fatal, msg: format!("invalid node"))?,
        }
    }

    fn as_mut_edge(&mut self) -> &mut Edge {
        use Node::{Child, Descendant, End, Pattern, Select, Sibling, Twin};

        match self {
            Pattern(edge, _) => edge,
            Select { edge, .. } => edge,
            Twin { edge, .. } => edge,
            Sibling { edge, .. } => edge,
            Child { edge, .. } => edge,
            Descendant { edge, .. } => edge,
            End(_) => unreachable!(),
        }
    }

    fn pos_to_text(&mut self, text: &str) -> Result<()> {
        use Node::{Child, Descendant, End, Pattern, Select, Sibling, Twin};

        match self {
            Pattern(edge, _) => edge.pos_to_text(text),
            Select { edge, .. } => edge.pos_to_text(text),
            Twin { edge, .. } => edge.pos_to_text(text),
            Sibling { edge, .. } => edge.pos_to_text(text),
            Child { edge, .. } => edge.pos_to_text(text),
            Descendant { edge, .. } => edge.pos_to_text(text),
            End(_) => Ok(()),
        }
    }
}

impl Node {
    fn compile_style<'a>(
        ts_node: ts::Node<'a>,
        text: &str,
        tc: &mut ts::TreeCursor<'a>,
        scheme: &ColorScheme,
    ) -> Result<Node> {
        let canvas = scheme.to_style(Highlight::Canvas);
        let style = match ts_node.kind() {
            "highlight" => {
                let mut cont = Span::from_node(&ts_node.child(0).unwrap());
                cont.pos_to_text(text)?;
                match cont {
                    Span::Text(hl) => {
                        let hl: Highlight = TryFrom::try_from(hl.as_str())?;
                        Ok(scheme.to_style(hl))
                    }
                    _ => err_at!(Fatal, msg: format!("unexpected style")),
                }?
            }
            "properties" => {
                let mut style: Style = Default::default();
                for nprop in ts_node.child(1).unwrap().children(tc) {
                    let nprop = nprop.child_by_field_name("property").unwrap();
                    let mut cont = Span::from_node(&nprop.child(2).unwrap());
                    cont.pos_to_text(text)?;
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
        mut next: Node,
        tc: &mut ts::TreeCursor<'a>,
    ) -> Result<Node> {
        match ts_node.child_count() {
            0 => err_at!(Fatal, msg: format!("unexpected node")),
            1 => Self::compile_sel(ts_node.child(0).unwrap(), next, tc),
            _ => {
                let mut cs: Vec<ts::Node> = ts_node.children(tc).collect();
                cs.reverse();
                let mut iter = cs.into_iter();
                next = Self::compile_sel(iter.next().unwrap(), next, tc)?;
                for child in iter {
                    wrap_edge!(next.as_mut_edge(), Descendant)?;
                    next = Self::compile_sel(child, next, tc)?;
                }
                Ok(next)
            }
        }
    }

    fn compile_sel<'a>(
        ts_node: ts::Node<'a>,
        mut next: Node,
        tc: &mut ts::TreeCursor<'a>,
    ) -> Result<Node> {
        let cs: Vec<ts::Node> = ts_node.children(tc).collect();

        let chd = &cs[0];
        match chd.kind() {
            "sel_kind" => {
                let edge = Edge::Kind(Span::from_node(&chd));
                Ok(Node::Pattern(edge, Rc::new(next)))
            }
            "sel_field" => {
                let edge = Edge::Field(Span::from_node(&chd.child(1).unwrap()));
                Ok(Node::Pattern(edge, Rc::new(next)))
            }
            "sel_symbol_field" => {
                let edge = {
                    let ck = Span::from_node(&chd.child(0).unwrap());
                    let cf = Span::from_node(&chd.child(2).unwrap());
                    Edge::KindField(ck, cf)
                };
                Ok(Node::Pattern(edge, Rc::new(next)))
            }
            "sel_twins" => {
                next = Self::compile_sel(chd.child(2).unwrap(), next, tc)?;
                wrap_edge!(next.as_mut_edge(), Twin)?;
                Self::compile_sel(chd.child(0).unwrap(), next, tc)
            }
            "sel_siblings" => {
                next = Self::compile_sel(chd.child(2).unwrap(), next, tc)?;
                wrap_edge!(next.as_mut_edge(), Sibling)?;
                Self::compile_sel(chd.child(0).unwrap(), next, tc)
            }
            "sel_child" => {
                next = Self::compile_sel(chd.child(2).unwrap(), next, tc)?;
                wrap_edge!(next.as_mut_edge(), Child)?;
                Self::compile_sel(chd.child(0).unwrap(), next, tc)
            }
            kind => err_at!(Fatal, msg: format!("unexpected {}", kind)),
        }
    }
}

#[cfg(test)]
#[path = "tss_test.rs"]
mod tss_test;
