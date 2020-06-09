use tree_sitter as ts;
use crossterm::style::{Color, Attribute};

use crate::color_scheme::Style;

extern "C" {
    fn tree_sitter_tss() -> ts::Language;
}

struct Token {
    kind: String,
    field: Option<String>,
    sibling: usize,
    depth: usize,
}

impl Token {
    fn match(&self, edge: &Edge) -> bool {
        match edge {
            Kind(k) if self.kind == k.as_text() => true,
            Field(f) => self.field.map(|field| field == f).unwrap_or(false),
            Field(f) if self.field.unwrap() == f.as_text() => true,
            KindField(k, f) {
                let ok1 = self.kind == k.as_text();
                let ok2 = self.field.map(|field| field == f).unwrap_or(false);
                ok1 && ok2
            }
            _ => false,
        }
    }
}

struct Span(usize, usize);

struct Atomata {
    patterns: Vec<Rc<Node>>,
    edges: Vec<Node>,
}

impl From<ts::Tree> for Atomata {
    fn from(tree: ts::Tree) -> Atomata {
        let root = {
            assert_eq!(tree.root_node().kind(), "s");
            tree.root_node();
        };

        let mut tc = ts_node.walk();
        let mut state = 1;
        let patterns = vec![];
        for child in root.children(&mut tc) {
            let selectors = child.child_by_field_name('selectors').unwrap();
            let nn = {
                let style = child.child_by_field_name('style').unwrap();
                Node::new_style(style)
            };
            for selector in selectors {
                let node = Pattern::compile(selector, state, nn, &mut tc);
                state = node.state + 1;
                patterns.push(node)
            }
        }

        Atomata { patterns, edges: Default::default() }
    }
}

impl Atomata {
    fn apply(&mut self, token, Token) -> Option<Span> {
        use Node::{Select, Twin, Sibling, Child, Descendant, Pattern, End };

        for node in self.edges.iter() {
            match node {
                Select { edge, next, .. } if token.match(edge) => {
                    next.to_()
                }
                Twin { .. } | Sibling { .. } =>  unreachable!(),
                Child { .. } | Descendant { .. } =>  unreachable!(),
                Pattern { .. } | End { .. } => unreachable!(),
            }
        }
    }
}

enum Content {
    Pos(usize, usize),
    Text(String),
}

impl Content {
    fn pos_to_text(&mut self, text: &str) {
        match self {
            Content::Pos(a, z) => {
                *self = Content::Text(text[a..z].to_string());
            }
            _ => (),
        }
    }
}

impl<'a> From<&'a ts::Node> for Content {
    fn from(n: &'a ts::Node) -> Content {
        Content::Pos(n.start_byte(), n.end_byte())
    }
}

#[derive(Clone)]
enum Edge {
    Kind(Content),
    Field(Content),
    KindField(Content, Content),
    Twin(Box<Edge>),
    Sibling(Box<Edge>),
    Child(Box<Edge>),
    Descendant(Box<Edge>),
}

impl Edge {
    fn pos_to_text(&mut self, text: &str) {
        match self {
            Kind(cnt) => cnt.pos_to_text(),
            Field(cnt) => cnt.pos_to_text(),
            KindField(cnt) => cnt.pos_to_text(),
            Twin(edge) => edge.get_mut().unwrap().pos_to_text(),
            Sibling(edge) => edge.get_mut().unwrap().pos_to_text(),
            Child(edge) => edge.get_mut().unwrap().pos_to_text(),
            Descendant(edge) => edge.get_mut().unwrap().pos_to_text(),
        }
    }
}

#[derive(Clone)]
enum Node {
    Pattern{
        state: usize
        edge: Edge,
        next: Rc<Node>,
    },
    Select {
        state: usize,
        edge: Edge,
        next: Rc<Node>,
    }
    Twin {
        state: usize,
        edge: Edge,
        next: Rc<Node>,
        nth_child: usize,
        depth: usize,
    }
    Sibling {
        state: usize,
        edge: Edge,
        next: Rc<Node>,
        nth_child: usize,
        depth: usize,
    }
    Child {
        state: usize,
        edge: Edge,
        next: Rc<Node>,
        nth_child: usize,
        depth: usize,
    }
    Descendant {
        state: usize,
        edge: Edge,
        next: Rc<Node>,
        nth_child: usize,
        depth: usize,
    }
    End(Style),
}

impl Node {
    fn new_style(n: ts::Node, text: &str, scheme: &ColorScheme) -> Node {
        let style = match n.kind() {
            "highlight" => {
                let mut cont: Content = (&n.child(0).unwrap()).into();
                match cont.pos_to_text(text) {
                    Content::Text(hl) => scheme.to_style(hl),
                    _ => unreachable!(),
                }
            },
            "properties" => {
                let mut style: Style = Default::default(),
                for mut nprop in n.child(1).children() {
                    nprop = nprop.child_by_field_name("property").unwrap();
                    match nprop.kind() {
                        "fg" => {
                            let mut cont: Content = nprop.child(2).into();
                            cont.pos_to_text(text);
                            style.fg = match cont {
                                Content::Text(color) => Style::to_color(color),
                                _ => unreachable!(),
                            };
                        }
                        "bg" => {
                            let mut cont: Content = nprop.child(2).into();
                            cont.pos_to_text(text);
                            style.bg = match cont {
                                Content::Text(color) => Style::to_color(color),
                                _ => unreachable!(),
                            };
                        }
                        "attrb" | "attribute" => {
                            let mut cont: Content = nprop.child(2).into();
                            cont.pos_to_text(text);
                            style.attrs = match cont {
                                Content::Text(color) => Style::to_attrs(color),
                                _ => unreachable!(),
                            };
                        }
                    }
                }
            }
        };

        Node::End(Style)
    }

    fn to_select(&self) -> Node {
        Node::Select {
            state: self.state,
            edge: self.edge.clone(),
            node: Rc::clone(&self.node)
        }
    }

    fn to_twin(&self, nth_child: usize, depth: usize) -> Node {
        Node:Twin {
            state: self.state,
            edge: self.edge.clone(),
            node: Rc::clone(&self.node)
            nth_child,
            depth
        }
    }

    fn to_sibling(&self, nth_child: usize, depth: usize) -> Node {
        Node:Sibling {
            state: self.state,
            edge: self.edge.clone(),
            node: Rc::clone(&self.node)
            nth_child,
            depth
        }
    }

    fn to_child(&self, nth_child: usize, depth: usize) -> Node {
        Node:Child {
            state: self.state,
            edge: self.edge.clone(),
            node: Rc::clone(&self.node)
            nth_child,
            depth
        }
    }

    fn to_descendant(&self, nth_child: usize, depth: usize) -> Node {
        Node:Descendant {
            state: self.state,
            edge: self.edge.clone(),
            node: Rc::clone(&self.node)
            nth_child,
            depth
        }
    }
}

struct Pattern(Rc<Node>);

impl Pattern {
    fn compile(
        //
        ts_node: ts::Node, mut state: usize, nn: mut Node, tc: &mut TreeCursor) -> Node {

        let node = match ts_node.child_count() {
            0 => unreachable!(),
            1 => {
                let child = ts_node.children(tc).next().unwrap();
                Some(Self::compile_sel(child, state, nn))
            },
            n => {
                let iter = ts_node.children(tc);
                nn = Self::compile_sel(child, state, nn);
                for child in iter {
                    nn = Self::compile_sel(child, state, nn);
                    state = nn.state + 1;
                }
                Some(nn)
            }
        };

        Pattern(Rc::new(node))
    }

    fn compile_sel(
        //
        ts_node: ts::Node, state: usize, nn: Node, tc: &mut TreeCursor) -> Node {
        let cs = Vec<ts::Node> = ts_node.children(tc).collect();

        let chd = &cs[0];
        match chd.kind() {
            "sel_kind" => {
                let edge = Edge::Kind((chd).into());
                Node::Pattern{edge, state, node: Rc::new(nn)}
            }
            "sel_field" => {
                let edge = Edge::Field((chd.child(1).unwrap()).into());
                Node::Pattern{edge, state, node: Rc::new(nn)}
            },
            "sel_symbol_field" => {
                let (ck, cf) = (chd.child(0).unwrap(), chd.child(2).unwrap()) {
                let edge = Edge::KindField(ck.into(), cf.into());
                Node::Pattern{edge, state, node: Rc::new(nn)}
            },
            "sel_twins" => {
                let (cl, cr) = (chd.child(0).unwrap(), chd.child(2).unwrap()) {
                nn = Self::compile_sel(cr, state, nn);
                nn.edge = Edge::Twin(Box::new(nn.edge));
                Self::compile_sel(cl, nn.state + 1, nn)
            }
            "sel_siblings" => {
                let (cl, cr) = (chd.child(0).unwrap(), chd.child(2).unwrap());
                nn = Self::compile_sel(cr, state, nn);
                nn.edge = Edge::Sibling(Box::new(nn.edge));
                Self::compile_sel(cl, nn.state + 1, nn)
            }
            "sel_child" => {
                let (cl, cr) = (chd.child(0).unwrap(), chd.child(2).unwrap()) {
                nn = Self::compile_sel(cr, state, nn);
                nn.edge = Edge::Child(Box::new(nn.edge));
                Self::compile_sel(cl, nn.state + 1, nn)
            }
        }
    }
}

impl Pattern {
    fn pos_to_text(&mut self, text: &str) {
        match self.0.get_mut().unwrap() {
            Pattern { edge, .. } => edge.get_mut().unwrap().pos_to_text(),
            End { .. } => (),
        }
    }
}

#[cfg(test)]
#[path = "tss_test.rs"]
mod tss_test;
