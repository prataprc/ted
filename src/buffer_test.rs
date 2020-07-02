use super::*;

#[test]
fn test_newline() {
    let split_nl = |bytes: &[u8]| -> Vec<String> {
        let r = Rope::from_reader(bytes).unwrap();
        r.lines_at(0).map(|l| l.to_string()).collect()
    };

    let ss = split_nl("hello\nworld".as_bytes());
    assert_eq!(ss[0].as_str(), "hello\n", "{}", ss[0]);
    assert_eq!(ss[1].as_str(), "world", "{}", ss[1]);

    let ss = split_nl("hello\rworld".as_bytes());
    assert_eq!(ss[0].as_str(), "hello\r", "{}", ss[0]);
    assert_eq!(ss[1].as_str(), "world", "{}", ss[1]);

    let ss = split_nl("hello\r\nworld".as_bytes());
    assert_eq!(ss[0].as_str(), "hello\r\n", "{}", ss[0]);
    assert_eq!(ss[1].as_str(), "world", "{}", ss[1]);

    let ss = split_nl("hello\nworld\rhow\r\nare you".as_bytes());
    assert_eq!(ss[0].as_str(), "hello\n", "{}", ss[0]);
    assert_eq!(ss[1].as_str(), "world\r", "{}", ss[1]);
    assert_eq!(ss[2].as_str(), "how\r\n", "{}", ss[2]);
    assert_eq!(ss[3].as_str(), "are you", "{}", ss[3]);
}
