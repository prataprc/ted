use tree_sitter as ts;

use std::ops;

use super::*;

#[test]
fn test_parse() {
    let _p = new_parser();
}

//#[test]
//fn test_rule_color_name() {
//    let mut p = new_parser();
//
//    let test_cases = vec![
//        ("black", ops::Range { start: 0, end: 5 }),
//        ("darkgrey", ops::Range { start: 0, end: 8 }),
//        ("dark-grey", ops::Range { start: 0, end: 9 }),
//        ("dark_grey", ops::Range { start: 0, end: 9 }),
//        ("red", ops::Range { start: 0, end: 3 }),
//        ("darkred", ops::Range { start: 0, end: 7 }),
//        ("dark-red", ops::Range { start: 0, end: 8 }),
//        ("dark_red", ops::Range { start: 0, end: 8 }),
//        ("green", ops::Range { start: 0, end: 5 }),
//        ("darkgreen", ops::Range { start: 0, end: 9 }),
//        ("dark-green", ops::Range { start: 0, end: 10 }),
//        ("dark_green", ops::Range { start: 0, end: 10 }),
//        ("yellow", ops::Range { start: 0, end: 6 }),
//        ("darkyellow", ops::Range { start: 0, end: 10 }),
//        ("dark-yellow", ops::Range { start: 0, end: 11 }),
//        ("dark_yellow", ops::Range { start: 0, end: 11 }),
//        ("blue", ops::Range { start: 0, end: 4 }),
//        ("darkblue", ops::Range { start: 0, end: 8 }),
//        ("dark-blue", ops::Range { start: 0, end: 9 }),
//        ("dark_blue", ops::Range { start: 0, end: 9 }),
//        ("magenta", ops::Range { start: 0, end: 7 }),
//        ("darkmagenta", ops::Range { start: 0, end: 11 }),
//        ("dark-magenta", ops::Range { start: 0, end: 12 }),
//        ("dark_magenta", ops::Range { start: 0, end: 12 }),
//        ("cyan", ops::Range { start: 0, end: 4 }),
//        ("darkcyan", ops::Range { start: 0, end: 8 }),
//        ("dark-cyan", ops::Range { start: 0, end: 9 }),
//        ("dark_cyan", ops::Range { start: 0, end: 9 }),
//        ("white", ops::Range { start: 0, end: 5 }),
//        ("grey", ops::Range { start: 0, end: 4 }),
//    ];
//
//    for case in test_cases.into_iter() {
//        let tree = p.parse(case.0, None).unwrap();
//        let lang = tree.language();
//        // TODO: why is this version 11 ?
//        assert!(lang.version() == 11, "{:?} {}", case.0, lang.version());
//        assert!(
//            lang.node_kind_count() == 34,
//            "{:?} {}",
//            case.0,
//            lang.node_kind_count()
//        );
//
//        let root = tree.root_node();
//        {
//            let r = root.byte_range();
//            assert!(r.start == case.1.start, "{:?} {:?}", case.0, r);
//            assert!(r.end == case.1.end, "{:?} {:?}", case.0, r);
//        }
//        assert!(root.kind() == "color_name", "{:?} {}", case.0, root.kind());
//        assert!(
//            root.child_count() == 1,
//            "{:?} {}",
//            case.0,
//            root.child_count()
//        );
//        assert!(
//            root.named_child_count() == 0,
//            "{:?} {}",
//            case.0,
//            root.named_child_count()
//        );
//        assert!(root.is_error() == false);
//        assert!(root.is_missing() == false);
//        assert!(root.is_named() == true);
//        assert!(root.has_error() == false);
//
//        let node = root.child(0).unwrap();
//        {
//            let r = node.byte_range();
//            assert!(r.start == case.1.start, "{:?} {:?}", case.0, r);
//            assert!(r.end == case.1.end, "{:?} {:?}", case.0, r);
//        }
//        assert!(
//            node.child_count() == 0,
//            "{:?} {}",
//            case.0,
//            node.child_count()
//        );
//        assert!(node.is_error() == false);
//        assert!(node.is_missing() == false);
//        assert!(node.is_named() == false);
//        assert!(node.kind() == case.0, "{:?} {}", case.0, node.kind());
//        assert!(node.has_error() == false);
//
//        // TODO improvise this test case once full grammar is coded.
//    }
//}

fn new_parser() -> ts::Parser {
    let mut p = ts::Parser::new();
    let language = unsafe { tree_sitter_tss() };
    p.set_language(language).unwrap();
    p
}
