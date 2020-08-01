# write-json


Simple {dependency,trait,macro}-less JSON serialization


```rust
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
```
