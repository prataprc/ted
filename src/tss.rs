use tree_sitter as ts;

extern "C" {
    fn tree_sitter_tss() -> ts::Language;
}

#[cfg(test)]
#[path = "tss_test.rs"]
mod tss_test;
