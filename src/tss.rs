use crossterm::style::{Attribute, Color};
use tree_sitter as ts;

use std::{fmt, rc::Rc, result};

use crate::{color_scheme::Style, Error, Result};

extern "C" {
    fn tree_sitter_tss() -> ts::Language;
}

macro_rules! wrap_edge {
    ($self:expr, $varn:ident) => {
        match $self {
            Edge::Kind
            Edge::Field
            Edge::KindField
            _ => 
        }
        Edge::$varn(Box::new(
    };
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

//struct Atomata {
//    patterns: Vec<Rc<Node>>,
//    edges: Vec<Node>,
//}
//
//impl From<ts::Tree> for Atomata {
//    fn from(tree: ts::Tree) -> Atomata {
//        let root = {
//            assert_eq!(tree.root_node().kind(), "s");
//            tree.root_node();
//        };
//
//        let mut tc = ts_node.walk();
//        let mut state = 1;
//        let patterns = vec![];
//        for child in root.children(&mut tc) {
//            let selectors = child.child_by_field_name('selectors').unwrap();
//            let nn = {
//                let style = child.child_by_field_name('style').unwrap();
//                Node::new_style(style)
//            };
//            for selector in selectors {
//                let node = Pattern::compile(selector, state, nn, &mut tc);
//                state = node.state + 1;
//                patterns.push(node)
//            }
//        }
//
//        Atomata { patterns, edges: Default::default() }
//    }
//}

//impl Atomata {
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

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        use Edge::{Child, Descendant, Field, Kind, KindField, Sibling, Twin};

        match self {
            Kind(_) => write!(f, "e-kind"),
            Field(_) => write!(f, "e-field"),
            KindField(_, _) => write!(f, "e-kindf"),
            Twin(_) => write!(f, "e-twin"),
            Sibling(_) => write!(f, "e-sibling"),
            Child(_) => write!(f, "e-child"),
            Descendant(_) => write!(f, "e-descendant"),
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
    Pattern {
        state: usize,
        edge: Edge,
        next: Rc<Node>,
    },
    Select {
        state: usize,
        edge: Edge,
        next: Rc<Node>,
    },
    Twin {
        state: usize,
        edge: Edge,
        next: Rc<Node>,
        nth_child: usize,
        depth: usize,
    },
    Sibling {
        state: usize,
        edge: Edge,
        next: Rc<Node>,
        nth_child: usize,
        depth: usize,
    },
    Child {
        state: usize,
        edge: Edge,
        next: Rc<Node>,
        nth_child: usize,
        depth: usize,
    },
    Descendant {
        state: usize,
        edge: Edge,
        next: Rc<Node>,
        nth_child: usize,
        depth: usize,
    },
    End(Style),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        use Node::{Child, Descendant, End, Pattern, Select, Sibling, Twin};

        match self {
            Pattern { .. } => write!(f, "n-pattern"),
            Select { .. } => write!(f, "n-select"),
            Twin { .. } => write!(f, "n-twin"),
            Sibling { .. } => write!(f, "n-sibling"),
            Child { .. } => write!(f, "n-child"),
            Descendant { .. } => write!(f, "n-descendant"),
            End(_) => write!(f, "n-end"),
        }
    }
}

impl Node {
    //fn new_style(n: ts::Node, text: &str, scheme: &ColorScheme) -> Node {
    //    let style = match n.kind() {
    //        "highlight" => {
    //            let mut cont: Span = (&n.child(0).unwrap()).into();
    //            match cont.pos_to_text(text) {
    //                Span::Text(hl) => scheme.to_style(hl),
    //                _ => unreachable!(),
    //            }
    //        },
    //        "properties" => {
    //            let mut style: Style = Default::default(),
    //            for mut nprop in n.child(1).children() {
    //                nprop = nprop.child_by_field_name("property").unwrap();
    //                match nprop.kind() {
    //                    "fg" => {
    //                        let mut cont: Span = nprop.child(2).into();
    //                        cont.pos_to_text(text);
    //                        style.fg = match cont {
    //                            Span::Text(color) => Style::to_color(color),
    //                            _ => unreachable!(),
    //                        };
    //                    }
    //                    "bg" => {
    //                        let mut cont: Span = nprop.child(2).into();
    //                        cont.pos_to_text(text);
    //                        style.bg = match cont {
    //                            Span::Text(color) => Style::to_color(color),
    //                            _ => unreachable!(),
    //                        };
    //                    }
    //                    "attrb" | "attribute" => {
    //                        let mut cont: Span = nprop.child(2).into();
    //                        cont.pos_to_text(text);
    //                        style.attrs = match cont {
    //                            Span::Text(color) => Style::to_attrs(color),
    //                            _ => unreachable!(),
    //                        };
    //                    }
    //                }
    //            }
    //        }
    //    };

    //    Node::End(Style)
    //}

    fn to_state(&self) -> Option<usize> {
        use Node::{Child, Descendant, End, Pattern, Select, Sibling, Twin};

        match self {
            Pattern { state, .. } => Some(*state),
            Select { state, .. } => Some(*state),
            Twin { state, .. } => Some(*state),
            Sibling { state, .. } => Some(*state),
            Child { state, .. } => Some(*state),
            Descendant { state, .. } => Some(*state),
            End(_) => None,
        }
    }

    fn to_select(&self) -> Result<Node> {
        use Node::{Pattern, Select};

        match self {
            Pattern { state, edge, next } => Ok(Select {
                state: *state,
                edge: edge.clone(),
                next: Rc::clone(next),
            }),
            _ => err_at!(Fatal, msg: format!("invalid node"))?,
        }
    }

    fn to_twin(&self, nth_child: usize, depth: usize) -> Result<Node> {
        use Node::{Pattern, Twin};

        match self {
            Pattern { state, edge, next } => Ok(Twin {
                state: *state,
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
            Pattern { state, edge, next } => Ok(Sibling {
                state: *state,
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
            Pattern { state, edge, next } => Ok(Child {
                state: *state,
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
            Pattern { state, edge, next } => Ok(Descendant {
                state: *state,
                edge: edge.clone(),
                next: Rc::clone(next),
                nth_child,
                depth,
            }),
            _ => err_at!(Fatal, msg: format!("invalid node"))?,
        }
    }
}

struct Pattern(Rc<Node>);

impl Pattern {
    //fn compile(
    //    //
    //    ts_node: ts::Node, mut state: usize, mut nn: Node, tc: &mut TreeCursor) -> Result<Node> {

    //    let node = match ts_node.child_count() {
    //        0 => unreachable!(),
    //        1 => {
    //            let child = ts_node.children(tc).next().unwrap();
    //            Self::compile_sel(child, state, nn)?
    //        },
    //        n => {
    //            let iter = ts_node.children(tc);
    //            nn = Self::compile_sel(child, state, nn)?;
    //            for child in iter {
    //                nn.edge = Edge::Descendant(Box::new(nn.edge));
    //                nn = Self::compile_sel(child, state, nn)?;
    //                state = nn.state + 1;
    //            }
    //            nn
    //        }
    //    };

    //    Ok(Pattern(Rc::new(node)))
    //}

    fn compile_sel(
        ts_node: ts::Node,
        state: usize,
        mut next: Node,
        tc: &mut ts::TreeCursor,
    ) -> Result<Node> {
        let cs: Vec<ts::Node> = ts_node.children(tc).collect();

        let chd = &cs[0];
        match chd.kind() {
            "sel_kind" => {
                let edge = Edge::Kind(Span::from_node(&chd));
                Ok(Node::Pattern {
                    edge,
                    state,
                    next: Rc::new(next),
                })
            }
            "sel_field" => {
                let edge = {
                    let cf = chd.child(1).unwrap();
                    Edge::Field(Span::from_node(&cf))
                };
                Ok(Node::Pattern {
                    edge,
                    state,
                    next: Rc::new(next),
                })
            }
            "sel_symbol_field" => {
                let edge = {
                    let ck = Span::from_node(&chd.child(0).unwrap());
                    let cf = Span::from_node(&chd.child(2).unwrap());
                    Edge::KindField(ck, cf)
                };
                Ok(Node::Pattern {
                    edge,
                    state,
                    next: Rc::new(next),
                })
            }
            "sel_twins" => {
                next = {
                    let cr = chd.child(2).unwrap();
                    Self::compile_sel(cr, state, next, tc)?
                };
                next.edge = Edge::Twin(Box::new(next.edge));

                let cl = chd.child(0).unwrap();
                let state = next.to_state().unwrap_or(state + 1) + 1;
                Self::compile_sel(cl, state, next, tc)
            }
            "sel_siblings" => {
                next = {
                    let cr = chd.child(2).unwrap();
                    Self::compile_sel(cr, state, next, tc)?
                };
                next.edge = Edge::Sibling(Box::new(next.edge));

                let cl = chd.child(0).unwrap();
                let state = next.to_state().unwrap_or(state + 1) + 1;
                Self::compile_sel(cl, state, next, tc)
            }
            "sel_child" => {
                next = {
                    let cr = chd.child(2).unwrap();
                    Self::compile_sel(cr, state, next, tc)?
                };
                next.edge = Edge::Child(Box::new(next.edge));

                let cl = chd.child(0).unwrap();
                let state = next.to_state().unwrap_or(state + 1) + 1;
                Self::compile_sel(cl, state, next, tc)
            }
        }
    }
}

//impl Pattern {
//    fn pos_to_text(&mut self, text: &str) {
//        match self.0.get_mut().unwrap() {
//            Pattern { edge, .. } => edge.get_mut().unwrap().pos_to_text(),
//            End { .. } => (),
//        }
//    }
//}

#[cfg(test)]
#[path = "tss_test.rs"]
mod tss_test;
