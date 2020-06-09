use tree_sitter as ts;

extern "C" {
    fn tree_sitter_tss() -> ts::Language;
}

struct Token {
    kind: String,
    field: Option<String>,
    sibling: usize,
    depth: usize,
}

#[derive(Clone)]
enum Syntax {
    Style(Style),
    Highlight(String),
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
        node: Rc<Node>,
    },
    Select {
        state: usize,
        edge: Edge,
        node: Rc<Node>,
    }
    Twin {
        state: usize,
        edge: Edge,
        node: Rc<Node>,
        nth_child: usize,
        depth: usize,
    }
    Sibling {
        state: usize,
        edge: Edge,
        node: Rc<Node>,
        nth_child: usize,
        depth: usize,
    }
    Child {
        state: usize,
        edge: Edge,
        node: Rc<Node>,
        nth_child: usize,
        depth: usize,
    }
    Descendant {
        state: usize,
        edge: Edge,
        node: Rc<Node>,
        nth_child: usize,
        depth: usize,
    }
    End(Syntax),
}

impl Node {
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
        for child in root.children(&mut tc) {
            let selectors = child.child(0).unwrap();
            let selector = selectors.child(0);
            for selector in selectors.children(&mut tc) {
            }
        }
    }
}

struct Pattern(Rc<Node>);

impl Pattern {
    fn compile(ts_node: ts::Node, mut state: usize, nn: mut Node) -> Node {
        let mut tc = ts_node.walk();

        let node = match ts_node.child_count() {
            0 => unreachable!(),
            1 => {
                let child = ts_node.children(&mut tc).next().unwrap();
                Some(Self::compile_sel(child, state, nn))
            },
            n => {
                let iter = ts_node.children(&mut tc);
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

    fn compile_sel(ts_node: ts::Node, state: usize, nn: Node) -> Node {
        let mut tc = ts_node.walk();
        let cs = Vec<ts::Node> = ts_node.children(&mut tc).collect();

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
