use tree_sitter as ts;

extern "C" {
    fn tree_sitter_tss() -> ts::Language;
}

enum Syntax {
    Style(Style),
    Highlight(String),
}

enum Edge {
    E,
    Sym(String),
    Fld(String),
    SymFld(String, String),
    Next,
    Young,
    Child,
    Descendant,
}

enum State {
    Start,
    Select,
    Next,
    Young,
    Child,
    Descendant,
    End(Syntax),
}

struct Node {
    state: State,
    transitions: Vec<(Edge, Rc<Node>)>,
    depth: usize,
}

struct Atomata {
    atomata: Rc<Node>,
    nodes: Vec<Rc<Node>>,
}

impl Atomata {
    fn new() -> Atomata {
        let start = Node::Start(vec![]);
        Atomata { s: start }
    }
}

#[cfg(test)]
#[path = "tss_test.rs"]
mod tss_test;
