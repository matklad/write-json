#[test]
fn smoke() {
    let mut buf = String::new();

    {
        let mut obj = write_json::object(&mut buf);
        obj.string("name", "Peter").number("favorite number", 92.0);
        obj.array("films")
            .string("Drowning By Numbers")
            .string("A Zed & Two Noughts");
        obj.null("suitcase");
    }

    assert_eq!(
        buf,
        r#"{"name":"Peter","favorite number":92,"films":["Drowning By Numbers","A Zed & Two Noughts"],"suitcase":null}"#
    )
}

#[test]
fn string_escaping() {
    let mut buf = String::new();
    {
        write_json::array(&mut buf)
            .string("")
            .string("'")
            .string("\"")
            .string("\\")
            .string("hello world")
            .string(" \r\n\t\\ \\r\\n\\t")
            .string("❤😂")
            .string("\x00\x07\x1F\x20\x7E\x7F\u{80}\u{9f}!");
    }
    let strings = buf.replace(|c: char| "[],".contains(c), "\n");
    let expected = r#"
""
"'"
"\""
"\\"
"hello world"
" \r\n\t\\ \\r\\n\\t"
"❤\u0020"
"\u0000\u0070\u00F1 ~\u00F7\u0008\u00F9!"
"#;

    assert_eq!(strings, expected);
}
