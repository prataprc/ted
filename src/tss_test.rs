use tree_sitter as ts;

use std::{fs, path::PathBuf};

use super::*;

#[test]
fn test_parse() {
    let _p = new_parser();
}

#[test]
fn test_automata() {
    use std::str::from_utf8;

    let text = {
        let mdir = env!("CARGO_MANIFEST_DIR");
        let fpath: PathBuf = [mdir, "ts", "toml.tss"].iter().collect();
        let bytes = fs::read(fpath).unwrap();
        from_utf8(&bytes).unwrap().to_string()
    };
    let atm: Automata = text.parse().unwrap();
    println!("{}", atm);
}

fn new_parser() -> ts::Parser {
    let mut p = ts::Parser::new();
    let language = unsafe { tree_sitter_tss() };
    p.set_language(language).unwrap();
    p
}
